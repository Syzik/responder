import './assets/main.css'
import 'primevue/resources/themes/aura-dark-green/theme.css'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'

import { createApp } from 'vue'
import App from './App.vue'

import PrimeVue from 'primevue/config'
import Button from "primevue/button"
import Dropdown from "primevue/dropdown"
import FloatLabel from "primevue/floatlabel"
import DataTable from "primevue/datatable"
import Column from "primevue/column"
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import InputSwitch from 'primevue/inputswitch'
import Tooltip from 'primevue/tooltip';

import { install as VueMonacoEditorPlugin } from '@guolao/vue-monaco-editor'

createApp(App)
  .component('Button', Button)
  .component('Dropdown', Dropdown)
  .component('FloatLabel', FloatLabel)
  .component('DataTable', DataTable)
  .component('Column', Column)
  .component('InputNumber', InputNumber)
  .component('InputText', InputText)
  .component('Textarea', Textarea)
  .component('InputSwitch', InputSwitch)
  .directive('tooltip', Tooltip)
  .use(PrimeVue, { ripple: true })
  .use(VueMonacoEditorPlugin, {
    paths: {
      vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.43.0/min/vs'
    },
  })
  .mount('#app')
