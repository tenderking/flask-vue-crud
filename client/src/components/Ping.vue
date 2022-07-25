<template>
  <div>
    <b-button class="btn btn-primary" @click="modalShow = true"
      >Launch demo modal</b-button
    >

    <b-modal id="modal-1" title="BootstrapVue" v-if="modalShow">
      <p class="my-4">Hello from modal!</p>
    </b-modal>
  </div>
</template>
<script lang="ts" setup>
import axios from "axios";
import { ref, onMounted } from "vue";

const msg = ref("hello");
const modalShow = ref(false);
const getMessage = () => {
  const path = "http://localhost:5000/ping";
  axios
    .get(path)
    .then((res) => {
      msg.value = res.data;
    })
    .catch((error) => {
      console.error(error);
    });
};

onMounted(() => {
  console.log(`the component is now mounted.`);
  getMessage();
});
</script>
