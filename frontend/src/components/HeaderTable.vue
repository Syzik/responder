<script setup>
import { ref } from 'vue'

const headers = ref([{ name: 'Content-Type', value: 'application/json' }])

const onCellEditComplete = (event) => {
  const { data, newValue, field } = event

  if (newValue.trim()) {
    data[field] = newValue
  } else {
    data[field] = ''
  }
}
</script>

<template>
  <h2>Headers</h2>
  <DataTable
    :value="headers"
    editMode="cell"
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
    <Column class="text-wrap text-primary" field="name" header="Name" style="width: 50%">
      <template #editor="{ data, field }">
        <InputText v-model="data[field]" />
      </template>
    </Column>
    <Column class="text-wrap" field="value" header="Value" style="width: 50%">
      <template #editor="{ data, field }">
        <Textarea v-model="data[field]" autoResize rows="1" cols="30" />
      </template>
    </Column>
  </DataTable>
</template>
