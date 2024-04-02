<script setup>
import { ref } from 'vue'

const languages = [
  { mime: 'text/html', name: 'html' },
  { mime: 'text/javascript', name: 'javascript' },
  { mime: 'application/json', name: 'json' },
  { mime: 'application/xml', name: 'xml' },
  { mime: 'text/css', name: 'css' }
]
const selected = ref(languages[0])
const filtered = ref(languages)

function onChange() {
  const value = typeof selected.value === 'object' ? selected.value.mime : selected.value
  console.log(value)

  filtered.value = languages.filter((language) =>
    language.mime.toLowerCase().includes(value.toLowerCase())
  )
}
</script>

<template>
  <Dropdown
    v-model="selected"
    editable
    :options="filtered"
    optionLabel="mime"
    placeholder="Choose a MIME type"
    class="w-full md:w-14rem"
    @change="onChange"
  />
</template>
