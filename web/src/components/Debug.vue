<template>
  <div class="debug">
    <button @click="toggleDebugMode">Debug mode</button>
    <div v-if="debugMode" class="debug__panel debug-panel">
      <h1>Debug me</h1>
      <div class="debug-panel romdata">
        <p v-for="(line, i) in hexRom" :key="i">
          <span v-for="(bytes, j) in line" :key="j">{{bytes}}</span>
        </p>
      </div>
    </div>
  </div>
</template>

<script>

import { mapGetters, mapActions } from 'vuex'

export default {
  name: 'Debug',
  data () {
    return {
      romName: ''
    }
  },
  computed: {
    ...mapGetters(['debugMode', 'rom']),
    hexRom() {
      return Array.from(this.rom)
        .map(number => number.toString(16))
        .map((byte, index, array) => index % 2 === 0 ? `${array[index]}${array[index + 1]}` : '') // combine 2 bytes
        .filter(bytes => bytes !== '' && bytes !== '00') // filter 'empty' bytes
        .map(bytes => bytes.length === 3 ? `0${bytes}` : bytes) // add padding
        .reduce((previous, current, index) => { // 16 bytes per line
          if (index % 8 === 0) {
            previous = [...previous, []]
          }
          const lastIndex = previous.length -1
          previous[lastIndex] = [...previous[lastIndex], current]
          return previous
        }, [])

    }
  },
  methods: {
    ...mapActions(['debugModeOn', 'debugModeOff']),
    toggleDebugMode() {
      this.debugMode ? this.debugModeOff() : this.debugModeOn();
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>
