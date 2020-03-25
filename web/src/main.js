import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

import("./wasm.js")
  .catch(e => console.error("Error importing `wasm.js`:", e));


Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
