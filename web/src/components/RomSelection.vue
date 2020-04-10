<template>
  <div class="rom-selection">
      <div class="rom-selection__select select">
        <select v-model="selectedRom" @change="romSelected">
          <option v-for="rom in roms" :key="rom.name" :value="rom.file">{{rom.name}}</option>
        </select>
      </div>
      <p class="rom-selection__text">or upload own ROM</p>
      <div class="rom-selection__upload">
        <RomUploader />
      </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex'
import RomUploader from '@/components/RomUploader.vue'

export default {
  name: 'RomSelection',
  components: {
    RomUploader
  },
  data () {
    return {
      roms: [
        {
          'name': 'Airplane',
          'file': 'Airplane.ch8'
        },
        {
          'name': 'Breakout',
          'file': 'Breakout.ch8'
        },
        {
          'name': 'Chip 8 Picture',
          'file': 'Chip8_Picture.ch8'
        },
        {
          'name': 'Guess',
          'file': 'Guess.ch8'
        },
        {
          'name': 'IBM Logo',
          'file': 'ibm_logo.ch8'
        },
        {
          'name': 'Landing',
          'file': 'Landing.ch8'
        },
        {
          'name': 'Maze',
          'file': 'maze.ch8'
        },
        {
          'name': 'Pong',
          'file': 'Pong.ch8'
        },
        {
          'name': 'Trip8 Demo',
          'file': 'Trip8_Demo.ch8'
        },
        {
          'name': 'Tron',
          'file': 'Tron.ch8'
        },
      ],
      selectedRom: 'Chip8_Picture.ch8'
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
  },
  mounted() {
    this.romSelected()
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

.rom-selection {
  display: flex;
  flex-direction: row;
}

</style>
