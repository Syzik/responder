<script setup>
import { ref, shallowRef } from 'vue'
import { SelectLanguage, HeaderTable, StatusField } from '@/components'

const MONACO_EDITOR_OPTIONS = {
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true
}

const code = ref('Hello, world!')
const editorRef = shallowRef()
const handleMount = (editor) => (editorRef.value = editor)

function formatCode() {
  editorRef.value?.getAction('editor.action.formatDocument').run()
}
</script>

<template>
  <!-- TODO: code font -->
  <div class="mx-4">
    <h1>Responder</h1>
    <div class="flex flex-wrap">
      <div class="flex-1 border-round mr-4">
        <vue-monaco-editor
          v-model:value="code"
          theme="vs-dark"
          language="html"
          :options="MONACO_EDITOR_OPTIONS"
          @mount="handleMount"
          height="80vh"
          class="mb-2"
        />
        <div class="mx-2">
          <hr />
          <div class="mx-2">
            <Button @click="formatCode">Format</Button>
            <SelectLanguage class="ml-2" />
          </div>
        </div>
      </div>
      <div class="flex-1">
        <StatusField />
        <HeaderTable />
      </div>
    </div>
  </div>
</template>

<style scoped></style>
