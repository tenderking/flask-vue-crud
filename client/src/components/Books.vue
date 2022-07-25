<template>
  <teleport to="#app">
    <b-modal
      id="book-modal"
      class="position-absolute top-50 start-50 translate-middle border border-primary bg-light p-3"
      title="Add a new book"
      v-if="modalShow"
      hide-footer
      style="z-index: 10o"
    >
      <!-- <b-form class="w-100"> -->
      <form class="w-100" @submit="onSubmit" @reset="onReset">
        <div
          class="d-flex p-2"
          id="form-title-group"
          label="Title:"
          label-for="form-title-input"
        >
          <input
            id="form-title-input"
            type="text"
            v-model="addBookForm.title"
            required
            placeholder="Enter title"
          />
        </div>
        <div
          class="d-flex p-2"
          id="form-author-group"
          label="Author:"
          label-for="form-author-input"
        >
          <input
            id="form-author-input"
            type="text"
            v-model="addBookForm.author"
            required
            placeholder="Enter author"
          />
        </div>

        <div class="d-flex ms-4 p-2 gap-3 form-check" id="form-read-group">
          <input
            class="form-check-input"
            type="checkbox"
            v-model="addBookForm.read"
            id="flexCheckChecked"
            checked
          />
          <label class="form-check-label" for="flexCheckChecked"> Read? </label>
        </div>
        <div class="btn-group" role="group">
          <button type="submit" class="btn btn-primary" variant="primary">Submit</button>
          <button type="reset" class="btn btn-danger" variant="danger">Reset</button>
        </div>
      </form>
    </b-modal>
    <b-modal
      id="book-modal"
      class="position-absolute top-50 start-50 translate-middle border border-primary bg-light p-3"
      title="Add a new book"
      v-if="updateModalShow"
      hide-footer
      style="z-index: 10o"
    >
      <!-- <b-form class="w-100"> -->
      <form class="w-100" @submit="onSubmitUpdate" @reset="onReset">
        <div
          class="d-flex p-2"
          id="form-title-group"
          label="Title:"
          label-for="form-title-input"
        >
          <input
            id="form-title-input"
            type="text"
            v-model="editForm.title"
            required
            placeholder="Enter title"
          />
        </div>
        <div
          class="d-flex p-2"
          id="form-author-group"
          label="Author:"
          label-for="form-author-input"
        >
          <input
            id="form-author-input"
            type="text"
            v-model="editForm.author"
            required
            placeholder="Enter author"
          />
        </div>

        <div class="d-flex ms-4 p-2 gap-3 form-check" id="form-read-group">
          <input
            class="form-check-input"
            type="checkbox"
            v-model="editForm.read"
            id="flexCheckChecked"
            checked
          />
          <label class="form-check-label" for="flexCheckChecked"> Read? </label>
        </div>
        <div class="btn-group" role="group">
          <button type="submit" class="btn btn-primary" variant="primary">Submit</button>
          <button type="reset" class="btn btn-danger" variant="danger">Reset</button>
        </div>
      </form>
    </b-modal>
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

const modalShow = ref(false);
const updateModalShow = ref(false);

let booksList = ref([] as any[]);
let addBookForm = ref({
  id: "",
  title: "",
  author: "",
  read: [],
});
let editForm = ref({
  id: "",
  title: "",
  author: "",
  read: [],
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
  addBookForm.value.read = [];

  editForm.value.id = "";
  editForm.value.title = "";
  editForm.value.author = "";
  editForm.value.read = [];
};

const onSubmit = (e: Event) => {
  e.preventDefault();
  const payload = {
    title: addBookForm.value.title,
    author: addBookForm.value.author,
    read: false,
  };
  addBook(payload);
  initForm();
  modalShow.value = false;
};
const onReset = (e: Event) => {
  e.preventDefault();
};
const editBook = (book: any) => {
  console.log("You have clicked on:", book.author);
  editForm.value = book;
  updateModalShow.value = true;
  console.log("received:", editForm.value);
};
const onSubmitUpdate = (evt: Event) => {
  evt.preventDefault();
  const payload = {
    title: editForm.value.title,
    author: editForm.value.author,
    read: editForm.value.read,
  };
  updateBook(payload, editForm.value.id);
};
const updateBook = (
  payload: { title: string; author: string; read: never[] },
  bookID: string
) => {
  const path = `http://localhost:5000/books/${bookID}`;
  axios
    .put(path, payload)
    .then(() => {
      getBooks();
    })
    .catch((error) => {
      // eslint-disable-next-line
      console.error(error);
      getBooks();
    });
};
</script>
