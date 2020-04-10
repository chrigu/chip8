import Vue from 'vue'
import Vuex from 'vuex'
import mapKey from '../keyboard.js'

Vue.use(Vuex)

let animationId = null;
let paused = false;

const renderLoopFactory = (chip8, callback) => {
  const renderLoop = () => {
    if (!paused) {
      chip8.tick();
      animationId = requestAnimationFrame(renderLoop);
      callback()
    }
  };

  return renderLoop
}

function commitChip8Internals(commit, chip8) {

  const relativePc = chip8.pc[0] - 512
  const pc = relativePc === 0 ? 0 : relativePc - 2

  commit('setPc', pc) // todo: get offset from chip
  commit('setV', chip8.v)
  commit('setStack', chip8.stack)
  commit('setSp', chip8.sp[0])
  commit('setI', chip8.i[0])
}


export default (chip8) => {

  const store = new Vuex.Store({
    state: {
      rom: null,
      debugMode: false,
      paused: false,
      pc: 0,
      v: [],
      stack: [],
      sp: 0,
      i: 0
    },
    getters: {
      debugMode: state => state.debugMode,
      rom: state => chip8.romToU8Array(state.rom),
      isPaused: state => state.paused,
      pc: state => state.pc,
      v: state => state.v,
      stack: state => state.stack,
      sp: state => state.sp,
      i: state => state.i,
    },
    mutations: {
      setRom(state, rom) {
        state.rom = rom;
      },
      setDebugMode(state, debugMode) {
        state.debugMode = debugMode
      },
      setPause(state, paused) {
        state.paused = paused
      },
      setPc(state, pc) {
        state.pc = pc
      },
      setV(state, v) {
        state.v = v
      },
      setStack(state, stack) {
        state.stack = stack
      },
      setSp(state, sp) {
        state.sp = sp
      },
      setI(state, i) {
        state.i = i
      }
    },
    actions: {
      setRom({commit}, file) {
        const reader = new FileReader();
        reader.onload = function(theFile) {
          commit('setRom', reader.result);
          chip8.loadRomFromFile(reader.result)
          paused = false
          commit('setPause', paused)
          renderLoop()
        }
        reader.readAsArrayBuffer(file);
      },
      debugModeOn({commit}) {
        commit('setDebugMode', true)
      },
      debugModeOff({commit}) {
        commit('setDebugMode', false)
      },
      pause({commit}) {
        paused = true
        commit('setPause', paused)
        animationId = null;
        commitChip8Internals(commit, chip8)
      },
      run({commit}) {
        paused = false
        commit('setPause', paused)
        renderLoop()
      },
      step({commit, state}) {
        chip8.tick();
        if (state.debugMode) {
          commitChip8Internals(commit, chip8)
        }
      },
      keydown(store, keyCode) {
        let chip8Key = mapKey(keyCode);
        if (chip8Key > 0) {
          chip8.keydown(chip8Key)
        }
      },
      keyup(store, keyCode) {
        let chip8Key = mapKey(keyCode);
        if (chip8Key > 0) {
          chip8.keyup(chip8Key)
        }
      }
    }
  })

  const tickWrapper = () => {
    store.dispatch('step')
  }

  const renderLoop = renderLoopFactory(chip8, tickWrapper);
  return store
}
