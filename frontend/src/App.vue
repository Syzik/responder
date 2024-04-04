<script setup>
import { ref, shallowRef } from 'vue'
import { SelectLanguage, HeaderTable, StatusField } from '@/components'

const previewChecked = ref(false)
const code = ref(`<h1>Hello, world!</h1>`)
const language = ref('html')
const url = ref(getFinalURL())

const MONACO_EDITOR_OPTIONS = {
  automaticLayout: true
}

const editorRef = shallowRef()
const handleMount = (editor) => {
  editorRef.value = editor
  editor.onKeyDown((e) => {
    if (e.ctrlKey && e.code === 'KeyS') {
      e.preventDefault()
      formatCode()
    }
  })
}

function formatCode() {
  console.log(editorRef.value)
  editorRef.value?.getAction('editor.action.formatDocument').run()
}

function getFinalURL() {
  return `/?body=${encodeURIComponent(code.value)}`
}
function copyFinalURL(text) {
  navigator.clipboard.writeText(location.origin + text)
}

addEventListener('beforeunload', (event) => {
  event.preventDefault()
})
</script>

<template>
  <div class="mx-4">
    <h1>
      <a href="https://github.com/JorianWoltjer/responder" target="_blank">
        <img src="./favicon.svg" class="image-in-text mr-1"
      /></a>
      Responder
    </h1>
    <div class="flex flex-wrap">
      <div class="flex-1 border-round mr-4 relative">
        <vue-monaco-editor
          v-model:value="code"
          theme="vs-dark"
          language="html"
          :options="MONACO_EDITOR_OPTIONS"
          @mount="handleMount"
          @change="url = getFinalURL()"
          :height="previewChecked ? '50vh' : '80vh'"
          class="mb-2"
        />
        <iframe
          :hidden="!previewChecked"
          :src="previewChecked ? url : '/?body='"
          sandbox="allow-scripts allow-forms"
          style="width: 100%; height: calc(30vh - 0.5rem)"
        ></iframe>
        <div class="mx-2">
          <hr />
          <div>
            <SelectLanguage
              v-model="language"
              @change="editorRef.getModel().setLanguage(language)"
            />
            <div class="inline absolute right-0 mx-2">
              <Button
                :icon="previewChecked ? 'pi pi-eye' : 'pi pi-eye-slash'"
                aria-label="Preview"
                :severity="previewChecked ? 'primary' : 'secondary'"
                @click="previewChecked = !previewChecked"
              />
            </div>
          </div>
        </div>
      </div>
      <div class="flex-1 flex flex-column">
        <StatusField />
        <HeaderTable class="mb-3" />
        <div class="mt-auto w-full">
          <InputText
            v-tooltip.bottom="{ value: 'Final URL' }"
            v-model="url"
            readonly
            placeholder="Disabled"
            class="mr-2"
            style="width: calc(100% - 3rem)"
            @focus="$event.target.select()"
          />
          <Button
            v-tooltip.focus.top="{
              value: 'Copied!',
              pt: {
                arrow: {
                  style: {
                    borderTopColor: 'var(--primary-color)'
                  }
                },
                text: 'bg-primary font-medium'
              }
            }"
            style="float: right"
            icon="pi pi-clipboard"
            severity="secondary"
            aria-label="Copy to Clipboard"
            @click="copyFinalURL(url)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.image-in-text {
  height: 25px;
}
</style>
