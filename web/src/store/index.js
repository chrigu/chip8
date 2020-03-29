import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

let animationId = null;

const renderLoopFactory = (chip8, callback) => {
  const renderLoop = () => {
    chip8.tick();
    animationId = requestAnimationFrame(renderLoop);
    callback()
  };

  return renderLoop
}


export default (chip8) => {

  const store = new Vuex.Store({
    state: {
      rom: null,
      debugMode: false,
      paused: false,
      pc: 0,
      v: []
    },
    getters: {
      debugMode: state => state.debugMode,
      rom: state => chip8.romToU8Array(state.rom),
      isPaused: state => state.paused,
      pc: state => state.pc,
      v: state => state.v,
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
      setPc(state, address) {
        state.pc = address
      },
      setV(state, address) {
        state.v = address
      }
    },
    actions: {
      setRom({commit}, file) {
        const reader = new FileReader();
        reader.onload = function(theFile) {
          commit('setRom', reader.result);
          chip8.loadRomFromFile(theFile, reader.result)
          commit('setPause', false)
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
        cancelAnimationFrame(animationId);
        animationId = null;
        commit('setPc', chip8.pc[0] - 512) // todo: get offset from chip
        commit('setV', chip8.v)
        commit('setPause', true)
      },
      run({commit}) {
        commit('setPause', false)
        renderLoop()
      },
      step({commit, state}) {
        chip8.tick();
        if (state.debugMode) {
          commit('setPc', chip8.pc[0] - 512) // todo: get offset from chip
          commit('setV', chip8.v)
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
