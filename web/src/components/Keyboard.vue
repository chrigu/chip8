<template>
  <div class="keyboard">
    <div v-for="(row, i) in keys" :key="i" class="keyboard__row row">
      <button v-for="key in row" :key="key" class="keyboard__key">{{key}}</button>
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
        ['1', '2', '3', '4'],
        ['Q', 'W', 'E', 'R'],
        ['A', 'S', 'D', 'F'],
        ['Y', 'X', 'C', 'V'],
      ]
    }
  },
  methods: {
    ...mapActions(['setRom']),
    romSelected() {
      const url = `/roms/${this.selectedRom}`;
      fetch(url)
      .then(data => data.blob())
      .then(data => this.setRom(data))
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>
