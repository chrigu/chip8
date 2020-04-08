<template>
  <div class="rom-selection">
    <div class="rom-list">
      <div class="select">
        <select v-model="selectedRom" @change="romSelected">
          <option v-for="rom in roms" :key="rom.name" :value="rom.file">{{rom.name}}</option>
        </select>
      </div>
    </div>
    <RomUploader />
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
          'name': 'Worm',
          'file': 'Worm.ch8'
        },
      ],
      selectedRom: 'Chip8_Picture.ch8'
    }
  },
  methods: {
    ...mapActions(['setRom']),
    romSelected(other) {
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
