#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;

// The type to represent the ID of a message.

// We're going to store all of the books here. No need for a DB.
#[derive(Serialize, Clone, Deserialize, Debug)]
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

// #[put("/<id>", data = "<book>")]
// fn update(id: Id, book: Json<Message<'_>>) -> Value {
//     if db.contains_key(&id) {
//         db.insert(id, msg.contents);
//         json!({ "status": "ok" })
//     } else {
//         json!({ "status": "error" })
//     }
// }
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
            .mount("/", routes![list_books, index, add_book])
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
