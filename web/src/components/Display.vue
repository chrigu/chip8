<template>
  <div class="chip8-display">

    <button id="tick">Tick</button>
    <button id="pause">Pause</button>
    <pre id="display"></pre>
    <canvas class="display"></canvas>
  </div>
</template>

<script>
export default {
  name: 'Display',
  mounted() {
    this.$chip8.initDisplay('display')
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
.display {
    width: 640px;
    height: 320px;
    background-color: black;
}
</style>
