<template>
  <teleport to="#app">
    <TheModal
      v-model:title-value="bookForm.title"
      v-model:author-value="bookForm.author"
      v-model:read-value="bookForm.read"
      @click="onSubmitFunction"
      v-on-click-outside="modalShow=false"
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
                  <button
                    type="button"
                    @click="deleteBook(book.id)"
                    class="btn btn-danger btn-sm"
                  >
                    Delete
                  </button>
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
import { computed, ref, onMounted } from "vue";
import Alert from "./Alert.vue";
import TheModal from "./TheModal.vue";
import { vOnClickOutside } from '@vueuse/components'

const modalShow = ref(false);

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
const updateActive = ref(false);

// AXIOS REST method

const getBooks = () => {
  const path = "http://localhost:5000/books";
  axios
    .get(path)
    .then((res) => {
      booksList.value = res.data.books;
    })
    .catch((error) => {
      console.error(error);
    });
};
onMounted(() => {
  getBooks();
});
const initForm = () => {
  addBookForm.value.title = "";
  addBookForm.value.author = "";
  addBookForm.value.read = [];

  editForm.value.id = "";
  editForm.value.title = "";
  editForm.value.author = "";
  editForm.value.read = [];
};

const onSubmit = () => {
  const payload = {
    title: addBookForm.value.title,
    author: addBookForm.value.author,
    read: false,
  };
  console.log("payload is", payload);
  addBook(payload);
  initForm();
  modalShow.value = false;
  updateActive.value = false;
};
// const onReset = (e: Event) => {
//   e.preventDefault();
//   initForm();
// };
const editBook = (book: any) => {
  console.log("You have clicked on:", book.id);
  editForm.value = book;
  modalShow.value = true;
  updateActive.value = true;
};
const onSubmitUpdate = () => {
  const payload = {
    id: editForm.value.id,
    title: editForm.value.title,
    author: editForm.value.author,
    read: editForm.value.read,
  };
  console.table(payload);
  updateBook(payload, editForm.value.id);
};

const addBook = (payload = {}) => {
  const path = "http://localhost:5000/books";
  axios
    .post(path, payload)
    .then(() => getBooks())
    .catch((error) => {
      console.error(error);
    });
};
const updateBook = (payload: {}, bookID: string) => {
  const path = `http://localhost:5000/books/${bookID}`;
  axios
    .put(path, payload)
    .then(() => {
      console.log(bookID);
      modalShow.value = false;
      message.value = "Book updated";
      getBooks();
    })
    .catch((error) => {
      // eslint-disable-next-line
      console.log(bookID);
      console.error(error);
    });
};
const deleteBook = (bookID: String) => {
  const path = `http://localhost:5000/books/${bookID}`;
  axios
    .delete(path)
    .then(() => {
      message.value = "Book removed";
      showMessage.value = true;
      getBooks();
    })
    .catch((error) => console.error(error));
};
const onSubmitFunction = computed(() => {
  return updateActive.value ? onSubmitUpdate : onSubmit;
});

const bookForm = computed(() => {
  return updateActive.value ? editForm.value : addBookForm.value;
});
</script>
