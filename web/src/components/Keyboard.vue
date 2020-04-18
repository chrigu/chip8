<template>
  <div class="keyboard">
    <div v-for="(row, i) in keys" :key="i" class="keyboard__row row">
      <button
        v-for="key in row"
        :key="key"
        class="keyboard__key button"
        @click="keyPressed(key)">{{key}}</button>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex'

export default {
  name: 'Keyboard',
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
.keyboard {
  &__row {
    margin-bottom: 22px;
  }
  &__key {
    background-color: #222222;
    color: rgb(76, 179, 91);
    border-color: rgb(76, 179, 91);
    width: 45px;
    height: 45px;
    margin-right: 11px;
    font-family: 'vt323', Helvetica, Arial, sans-serif;
    font-size: 24px;
    line-height: 1rem;;

    &:active {
      background-color: rgb(76, 179, 91);
      color: #222222;
    }
  }
}
</style>
