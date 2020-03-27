import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

import("./chip8.js")
.then(chip8 => {

  Vue.prototype.$chip8 = chip8;
  Vue.config.productionTip = false

  new Vue({
    router,
    store,
    render: h => h(App)
  }).$mount('#app')

})
.catch(e => console.error("Error importing `chip8.js`:", e));
