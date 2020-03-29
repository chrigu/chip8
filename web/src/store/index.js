import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

let animationId = null;

const renderLoopFactory = (chip8) => {
  const renderLoop = () => {
    // console.log(cpu.read_pc(), 'pc')
    chip8.tick();
    animationId = requestAnimationFrame(renderLoop);
  };

  return renderLoop
}


export default (chip8) => {

  const renderLoop = renderLoopFactory(chip8);

  return new Vuex.Store({
    state: {
      rom: null,
      debugMode: false,
      paused: false,
      pc: 0
    },
    getters: {
      debugMode: state => state.debugMode,
      rom: state => chip8.romToU8Array(state.rom),
      isPaused: state => state.isPaused,
      pc: state => state.pc
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
      }
    },
    actions: {
      setRom({commit}, file) {
        const reader = new FileReader();
        reader.onload = function(theFile) {
          commit('setRom', reader.result);
          chip8.loadRomFromFile(theFile, reader.result)
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
        commit('setPause', true)
      },
      step({commit}) {
        chip8.tick();
        console.log(chip8.pc[0])
        commit('setPc', chip8.pc[0] - 512) // todo: get offset from chip
      }
    }
  })
}
