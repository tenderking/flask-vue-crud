<template>
  <teleport to="#app">
    <TheModal
      v-model:title-value="addBookForm.title"
      v-model:author-value="addBookForm.author"
      v-model:read-value="addBookForm.read"
      :onSubmit="onSubmit"
      v-if="modalShow"
    />
  </teleport>
  <b-container>
    <div class="row">
      <div class="col-sm-10">
        <h1>Books</h1>
        <hr />
        <br /><br />
        <Alert :message="message" v-if="showMessage"> </Alert>
        <b-button class="btn btn-primary" @click="modalShow = true">Add Book</b-button>
        <br /><br />
        <table class="table table-hover">
          <thead>
            <tr>
              <th scope="col">Title</th>
              <th scope="col">Author</th>
              <th scope="col">Read?</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="book in booksList" :key="book.id">
              <td>{{ book.title }}</td>
              <td>{{ book.author }}</td>
              <td><span v-if="book.read"> Yes </span> <span v-else>No</span></td>
              <td>
                <div class="btn-group" role="group">
                  <button
                    type="button"
                    class="btn btn-warning btn-sm"
                    @click="editBook(book)"
                  >
                    Update
                  </button>
                  <button type="button" class="btn btn-danger btn-sm">Delete</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </b-container>
</template>
<script lang="ts" setup>
import axios from "axios";
import { ref, onMounted } from "vue";
import Alert from "./Alert.vue";
import TheModal from "./TheModal.vue";

const modalShow = ref(false);

let booksList = ref([] as any[]);
let addBookForm = ref({
  id: "",
  title: "",
  author: "",
  read: false,
});
let editForm = ref({
  id: "",
  title: "",
  author: "",
  read: false,
});

//Alert message
const message = ref("");
const showMessage = ref(false);
// AXIOS REST methods
const path = "http://localhost:5000/books";

const getBooks = () => {
  axios
    .get(path)
    .then((res) => {
      booksList.value = res.data.books;
      console.table(booksList.value);
    })
    .catch((error) => {
      console.error(error);
    });
};

onMounted(() => {
  getBooks();
});

const addBook = (payload = {}) => {
  axios
    .post(path, payload)
    .then(() => {
      getBooks();
      message.value = "Book added";
      showMessage.value = true;
    })
    .catch((error) => {
      console.error(error);
    });
};
const initForm = () => {
  addBookForm.value.title = "";
  addBookForm.value.author = "";
  addBookForm.value.read = false;

  editForm.value.id = "";
  editForm.value.title = "";
  editForm.value.author = "";
  editForm.value.read = false;
};

const onSubmit = () => {
  console.log("I am submitting");
  const payload = {
    title: addBookForm.value.title,
    author: addBookForm.value.author,
    read: false,
  };
  console.log("payload is", payload);
  addBook(payload);
  initForm();
  modalShow.value = false;
};
// const onReset = (e: Event) => {
//   e.preventDefault();
//   initForm();
// };
const editBook = (book: any) => {
  console.log("You have clicked on:", book.author);
  addBookForm.value = book;
  modalShow.value = true;
};
// const onSubmitUpdate = (evt: Event) => {
//   evt.preventDefault();
//   const payload = {
//     title: editForm.value.title,
//     author: editForm.value.author,
//     read: editForm.value.read,
//   };
//   updateBook(payload, editForm.value.id);
// };

// const updateBook = (
//   payload: { title: string; author: string; read: never[] },
//   bookID: string
// ) => {
//   const path = `http://localhost:5000/books/${bookID}`;
//   axios
//     .put(path, payload)
//     .then(() => {
//       getBooks();
//     })
//     .catch((error) => {
//       // eslint-disable-next-line
//       console.error(error);
//       getBooks();
//     });
// };
</script>
