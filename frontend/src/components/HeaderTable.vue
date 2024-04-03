<script setup>
import { ref } from 'vue'

const headers = ref([
  { name: 'Content-Type', value: 'application/json' },
  { name: '', value: '' }
])
const deleteIndex = ref(-1)

const onCellEditComplete = (event) => {
  const { data, newValue, field } = event

  if (newValue.trim()) {
    data[field] = newValue
  } else {
    data[field] = ''
  }
  // Remove empty headers and add one at the end
  headers.value = headers.value.filter((header) => header.name.trim() || header.value.trim())
  headers.value.push({ name: '', value: '' })
}
</script>

<template>
  <h2 class="mb-2 ml-1">Headers</h2>
  <DataTable
    :value="headers"
    editMode="cell"
    showGridlines
    resizableColumns
    @cell-edit-complete="onCellEditComplete"
    :pt="{
      table: { style: 'min-width: 20rem' },
      column: {
        bodycell: ({ state }) => ({
          class: [{ 'pt-0 pb-0': state['d_editing'] }]
        })
      }
    }"
  >
    <Column class="text-wrap text-primary h-3rem p-0" field="name" style="width: 50%">
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
            v-if="(data['name'] || data['value']) && deleteIndex === index"
            class="absolute right-50"
            severity="danger"
            text
            icon="pi pi-times"
            aria-label="Delete"
            @click="
              (e) => {
                onCellEditComplete({ data, field: 'name', newValue: '' })
                onCellEditComplete({ data, field: 'value', newValue: '' })
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
    <Column class="text-wrap h-3rem p-0" field="value" style="width: 50%">
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
</template>
