<script setup>
const model = defineModel()

const languages = ['html', 'javascript', 'json', 'xml', 'css']
</script>

<template>
  <Dropdown
    v-model="model"
    filter
    autoFilterFocus
    :options="languages"
    placeholder="Select Language"
    class="w-full md:w-14rem"
  />
</template>
