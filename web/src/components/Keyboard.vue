<template>
  <div class="keyboard">
    <div v-for="(row, i) in keys" :key="i" class="keyboard__row row">
      <button v-for="key in row" :key="key" class="keyboard__key" @click="keyPressed(key)">{{key}}</button>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex'

export default {
  name: 'Keyboard',
  components: {
    Keyboard
  },
  data() {
    return {
      keys: [
        ['1', '2', '3'],
        ['Q', 'W', 'E'],
        ['A', 'S', 'D'],
        ['Y', 'X', 'C'],
      ]
    }
  },
  methods: {
    ...mapActions(['keydown', 'keyup']),
    keyPressed(key) {
      this.keydown(key)
      setTimeout(() => this.keyup(key), 100) //simulate key press
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>
