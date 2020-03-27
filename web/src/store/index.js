import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default (chip8) => {
  return new Vuex.Store({
    state: {
      rom: null,
      debugMode: false
    },
    getters: {
      debugMode: state => {
        return state.debugMode
      },
      rom: state => {
        return state.rom
      }
    },
    mutations: {
      setRom(state, rom) {
        state.rom = rom;
      },
      setDebugMode(state, debugMode) {
        state.debugMode = debugMode
      }
    },
    actions: {
      setRom({commit}, file) {
        const reader = new FileReader();
        reader.onload = function(theFile) {
          commit('setRom', reader.result);
          chip8.loadRomFromFile(theFile, reader.result)
        }
        reader.readAsArrayBuffer(file);
      },
      debugModeOn({commit}) {
        commit('setDebugMode', true)
      },
      debugModeOff({commit}) {
        commit('setDebugMode', false)
      }
    }
  })
}
