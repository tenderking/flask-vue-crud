<template>
  <div>
    <b-alert variant="success" show>{{ message }}</b-alert>
    <br />
  </div>
</template>
<script lang="ts" setup>
import { defineProps } from "vue";
const props = defineProps({
  message: {
    type: String,
    required: true,
  },
});
</script>
