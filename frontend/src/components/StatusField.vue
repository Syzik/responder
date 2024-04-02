<script setup>
import { ref } from 'vue'
import { getReasonPhrase } from 'http-status-codes'

const status = ref(200)
function codeToReasonPhrase(code) {
  try {
    return `"${getReasonPhrase(code)}"`
  } catch (error) {
    return `???`
  }
}
</script>

<template>
  <strong class="mr-2 text-primary">Status:</strong>
  <InputNumber inputClass="status" highlightOnFocus :useGrouping="false" v-model="status" />
  - {{ codeToReasonPhrase(status) }}
</template>

<style>
.status {
  width: 6ch;
  text-align: center;
}
</style>
