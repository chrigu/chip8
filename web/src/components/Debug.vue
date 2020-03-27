<template>
  <div class="debug">
    <button @click="toggleDebugMode">Debug mode</button>
    <div v-if="debugMode" class="debug__panel debug-panel">
      <h1>Debug me</h1>
      <div class="debug-panel romdata">
        <p>{{hexRom}}</p>
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
      console.log(this.rom)
      return this.rom
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
