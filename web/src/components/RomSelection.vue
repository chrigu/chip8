<template>
  <div class="rom-selection">
    <div class="rom-list">
      <div class="select">
  <select>
    <option>Airplane</option>
    <option>Breakout</option>
    <option>Chip8</option>
    <option>Guess</option>
    <option>IBM Logo</option>
    <option>Landing</option>
    <option>Maze</option>
  </select>
</div>
    </div>
    <RomUploader />
  </div>
</template>

<script>

import RomUploader from '@/components/RomUploader.vue'

export default {
  name: 'RomSelection',
  components: {
    RomUploader
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>
