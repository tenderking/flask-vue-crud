<template>
  <b-modal
    id="book-modal"
    class="position-absolute top-50 start-50 translate-middle border border-primary bg-light p-3"
    title="Add a new book"
    hide-footer
    style="z-index: 10o"
  >
    <!-- <b-form class="w-100"> -->
    <form class="w-100" @submit.prevent="">
      <div
        class="d-flex p-2"
        id="form-title-group"
        label="Title:"
        label-for="form-title-input"
      >
        <input
          id="form-title-input"
          type="text"
          :value="titleValue"
          required
          placeholder="Enter title"
          @input="
						$emit(
							'update:titleValue',
							($event.target as HTMLInputElement).value
						)
					"
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
          :value="authorValue"
          @input="
						$emit(
							'update:authorValue',
							($event.target as HTMLInputElement).value
						)
					"
          required
          placeholder="Enter author"
        />
      </div>

      <div class="d-flex ms-4 p-2 gap-3 form-check" id="form-read-group">
        <input
          class="form-check-input"
          type="checkbox"
          :checked="readValue"
          @change="
						$emit('update:readValue', ($event.target as HTMLInputElement).value)
					"
        />
        <label class="form-check-label" for="flexCheckChecked"> Read? </label>
      </div>
      <div class="btn-group" role="group">
        <button
          class="btn btn-primary"
          variant="primary"
          @click="$emit('click')"
          type="submit"
        >
          Submit
        </button>
        <button type="reset" class="btn btn-danger" variant="danger">Reset</button>
      </div>
    </form>
  </b-modal>
</template>

<script lang="ts" setup>
defineProps({
  titleValue: { type: String },
  authorValue: { type: String },
  readValue: { type: null },
});
defineEmits(["update:titleValue", "update:authorValue", "update:readValue", "click"]);
</script>
