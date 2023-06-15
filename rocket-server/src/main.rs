#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

// The type to represent the ID of a message.

// We're going to store all of the books here. No need for a DB.
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
struct Book {
    id: Option<Id>,
    title: String,
    author: String,
    read: bool,
}
type Id = i32;
type BookList = Mutex<Vec<Book>>;
// type Books<'r> = &'r State<BookList>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/books")]
async fn list_books(book_list: &State<BookList>) -> Json<Vec<Book>> {
    let my_list = book_list.lock().await;
    println!("Received request: {:?}", book_list);
    Json((*my_list).clone())
}
#[post("/books", data = "<new_book>")]
async fn add_book(book_list: &State<BookList>, new_book: Json<Book>) -> Json<Vec<Book>> {
    let mut my_list = book_list.lock().await;
    (*my_list).push(new_book.into_inner());
    Json((*my_list).clone())
}

#[put("/books/<id>", data = "<updated_book>")]
async fn update_book(
    id: Id,
    updated_book: Json<Book>,
    book_list: &State<BookList>,
) -> Option<Json<Vec<Book>>> {
    let mut my_list = book_list.lock().await;
    let index = (*my_list).iter().position(|book| book.id == Some(id));
    if let Some(index) = index {
        (*my_list)[index] = updated_book.into_inner();
        Some(Json((*my_list).clone()))
    } else {
        None
    }
}
#[delete("/books/<id>")]
async fn delete_book(id: Id, book_list: &State<BookList>) -> Option<Json<Vec<Book>>> {
    let mut my_list = book_list.lock().await;
    let index = (*my_list).iter().position(|book| book.id == Some(id));
    if let Some(index) = index {
        (*my_list).remove(index);
        Some(Json((*my_list).clone()))
    } else {
        None
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket
            .mount(
                "/",
                routes![index, list_books, add_book, update_book, delete_book],
            )
            .register("/", catchers![not_found])
            .manage(BookList::new(vec![
                Book {
                    id: Some(1),
                    title: "Harry Potter and the Philosopher's Stone".to_string(),
                    author: "J.K. Rowling".to_string(),
                    read: true,
                },
                Book {
                    id: Some(2),
                    title: "Harry Potter and the Chamber of Secrets".to_string(),
                    author: "J.K. Rowling".to_string(),
                    read: false,
                },
            ]))
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_list_books() {
        let rocket = rocket::build().attach(stage());
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/books").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let books: Vec<Book> = response.into_json().expect("valid json");
        assert_eq!(books.len(), 2);
    }
    #[test]
    fn test_add_book() {
        let rocket = rocket::build().attach(stage());
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let new_book = Book {
            id: Some(3),
            title: "The Hobbit".to_string(),
            author: "J.R.R. Tolkien".to_string(),
            read: false,
        };
        let response = client.post("/books").json(&new_book).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let books: Vec<Book> = response.into_json().expect("valid json");
        assert_eq!(books.len(), 3);
        assert_eq!(books[2], new_book);
    }
    #[test]
    fn test_update_book() {
        let rocket = rocket::build().attach(stage());
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let updated_book = Book {
            id: Some(1),
            title: "The Lord of the Rings".to_string(),
            author: "J.R.R. Tolkien".to_string(),
            read: true,
        };
        let response = client.put("/books/1").json(&updated_book).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let books: Vec<Book> = response.into_json().expect("valid json");
        assert_eq!(books.len(), 2);
        assert_eq!(books[0], updated_book);
    }
    #[test]

    fn test_delete_book() {
        let rocket = rocket::build().attach(stage());
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/books").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let books: Vec<Book> = response.into_json().expect("valid json");
        assert_eq!(books.len(), 2);
        let _response = client.delete("/books/1").dispatch();
        let response = client.get("/books").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let books: Vec<Book> = response.into_json().expect("valid json");
        assert_eq!(books.len(), 1);
    }
}
