<template>
  <div class="debug">
    <div class="debug__panel debug-panel">
      <h1>Debugger</h1>
      <button v-if="!isPaused" @click="pause" class="button debug-button">Pause</button>
      <button v-if="isPaused" @click="run" class="button debug-button is-success">Run</button>
      <button v-if="isPaused" @click="step" class="button debug-button">Step</button>
      <div class="debug-panel">
        <div class="romdata">
          <p v-for="(line, i) in hexRom" :key="i" class="romdata__line">
            <span v-for="(opcode, j) in line" :key="j" :class="{'opcode--current': j + 8 * i === pc / 2}" class="opcode">{{opcode}}</span>
          </p>
        </div>
        <ul class="registers">
          <h2>V-Registers</h2>
          <div v-for="(register, j) in v" :key="j" class="registers__register" :class="{'registers__register--last': j === 7}">
            {{displayHex(j)}}: {{displayHex(register)}}
          </div>
        </ul>
        <ul class="registers">
          <h2>Stack</h2>
          <div v-for="(stack, j) in v" :key="j" class="registers__register" :class="{'registers__register--last': j === 7}">
            {{displayHex(j)}}: {{displayHex(stack)}}
          </div>
        </ul>
        <div class="single-value">
          pc: {{pc}}
        </div>
        <div class="single-value">
          i: {{i}}
        </div>
        <div class="single-value">
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
    ...mapGetters(['rom', 'pc', 'isPaused', 'v', 'stack', 'i', 'sp']),
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
    ...mapActions(['pause', 'step', 'run']),
    displayHex(value) {
      return value < 16 ? `0${value.toString(16)}` : value.toString(16)
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

  .debug-panel {
    font-size: 1.2rem;
    text-align: left;
  }

 .opcode {
   display: inline;
   margin-right: 1em;

   &--current {
     background-color: rgb(76, 179, 91);
     color: #222222;
   }
 }

 .romdata {
   background-color: #222222;
   color: rgb(76, 179, 91);
   margin-bottom: 2rem;
 }

 .registers {
   margin-bottom: 2rem;
   &__register {
     display: inline;

     &--last {
       margin-right: 100%;
     }
   }
 }

 .single-value {
   margin-bottom: 1rem;
 }

 .debug-button {
   margin-right: 1rem;
 }
</style>
