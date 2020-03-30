<template>
  <div class="debug">
    <button @click="toggleDebugMode">Debug mode</button>
    <div v-if="debugMode" class="debug__panel debug-panel">
      <h1>Debug me</h1>
      <button v-if="!isPaused" @click="pause">Pause</button>
      <button v-if="isPaused" @click="run">Run</button>
      <button v-if="isPaused" @click="step">Step</button>
      <div class="debug-panel">
        <div class="romdata">
          <p v-for="(line, i) in hexRom" :key="i" class="romdata__line">
            <span v-for="(opcode, j) in line" :key="j" :class="{'opcode--current': j + 8 * i === pc}" class="opcode">{{opcode}}</span>
          </p>
        </div>
        <ul class="v-registers">
          <div v-for="(register, j) in v" :key="j">
            v: {{j}}: {{register.toString(16)}}
          </div>
        </ul>
        <div class="pc">
          pc: {{pc}}
        </div>
        <ul class="stack">
          <div v-for="(stack, j) in v" :key="j">
            stack: {{j}}: {{stack.toString(16)}}
          </div>
        </ul>
        <div class="i">
          i: {{i}}
        </div>
        <div class="sp">
          sp: {{sp}}
        </div>
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
    ...mapGetters(['debugMode', 'rom', 'pc', 'isPaused', 'v', 'stack', 'i', 'sp']),
    hexRom() {
      return Array.from(this.rom)
        .map(number => number.toString(16))
        .map((byte, index, array) => index % 2 === 0 ? `${array[index]}${array[index + 1]}` : '') // combine 2 bytes to opcode
        .filter(opcode => opcode !== '' && opcode !== '00') // filter 'empty' opcodes
        .map(opcode => opcode.length === 3 ? `0${opcode}` : opcode) // add padding
        .reduce((previous, current, index) => { // 8 opcodes per line
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
    ...mapActions(['debugModeOn', 'debugModeOff', 'pause', 'step', 'run']),
    toggleDebugMode() {
      this.debugMode ? this.debugModeOff() : this.debugModeOn();
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
 .opcode {

   display: inline;
   margin-right: 1em;

   &--current {
     background-color: rgb(76, 179, 91);
     color: #111111;
   }
 }

 .romdata {

   padding-left: 5rem;
   text-align: left;
   background-color: #111111;
   color: rgb(76, 179, 91);

 }
</style>
