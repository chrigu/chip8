<template>
  <div class="chip8">
    <input type="file" id="rom" name="rom" />
    <output id="list"></output>

    <button id="tick">Tick</button>
    <button id="pause">Pause</button>
    <pre id="display"></pre>
    <canvas class="display"></canvas>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld'
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
