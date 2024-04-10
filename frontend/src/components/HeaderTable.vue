<script setup>
import { ref, watchEffect } from 'vue'
import { CORS_HEADERS } from '@/utils/constants'

const model = defineModel()
const headers = ref(new Map(model.value))

watchEffect(() => {
  headers.value = new Map(model.value)
})

const deleteIndex = ref(-1)

const reloadHeaders = () => {
  headers.value = new Map(
    Array.from(headers.value).filter(([name, value]) => name.trim() || value.trim())
  )
  if (!headers.value.has('')) {
    headers.value.set('', '')
  }

  model.value = headers.value
}

const editHeaders = (event) => {
  const { data, newValue, field } = event

  if (data[field] === newValue) {
    return
  }

  if (field === '1') {
    headers.value.set(data['0'], newValue)
  }
  if (field === '0') {
    const array = Array.from(headers.value)
    headers.value.delete(data['0'])
    if (newValue.trim()) {
      const index = array.findIndex(([name]) => name === data['0'])
      array.splice(index, 1, [newValue, data['1']])
      headers.value = new Map(array)
    }
  }

  reloadHeaders()
}

const removeHeader = (name) => {
  headers.value.delete(name)
  reloadHeaders()
}

const addCORSHeaders = () => {
  for (const [name, value] of Object.entries(CORS_HEADERS)) {
    headers.value.set(name, value)
  }

  reloadHeaders()
}
</script>

<template>
  <div>
    <div class="my-3 mx-1">
      <h2 class="inline">Headers</h2>
      <Button
        label="CORS"
        title="Add allow-all CORS headers"
        severity="primary"
        rounded
        size="small"
        class="font-bold mb-1"
        style="float: right"
        @click="addCORSHeaders"
      />
    </div>
    <DataTable
      :value="Array.from(headers)"
      editMode="cell"
      showGridlines
      resizableColumns
      @cell-edit-complete="editHeaders"
      :pt="{
        table: { style: 'min-width: 20rem' },
        column: {
          bodycell: ({ state }) => ({
            class: [{ 'pt-0 pb-0': state['d_editing'] }]
          })
        }
      }"
    >
      <Column class="text-wrap text-primary h-3rem p-0" field="0" style="width: 50%">
        <template #header>
          <span class="m-2">Name</span>
        </template>
        <template #body="{ data, field, index }">
          <div
            class="h-full flex justify-content-center flex-column"
            @mouseenter="deleteIndex = index"
            @mouseleave="deleteIndex = -1"
          >
            <span class="m-2" v-text="data[field]" />
            <Button
              v-if="(data[0] || data[1]) && deleteIndex === index"
              class="absolute right-50"
              severity="danger"
              text
              icon="pi pi-times"
              aria-label="Delete"
              @click="
                (e) => {
                  removeHeader(data[0])
                  e.stopPropagation()
                }
              "
            />
          </div>
        </template>
        <template #editor="{ data, field }">
          <InputText class="w-full" v-model="data[field]" @mouseleave="deleteIndex = -1" />
        </template>
      </Column>
      <Column class="text-wrap h-3rem p-0" field="1" style="width: 50%">
        <template #header>
          <span class="m-2">Value</span>
        </template>
        <template #body="{ data, field }">
          <span class="m-2" v-text="data[field]" />
        </template>
        <template #editor="{ data, field }">
          <Textarea class="w-full" v-model="data[field]" autoResize rows="1" cols="30" />
        </template>
      </Column>
    </DataTable>
  </div>
</template>
