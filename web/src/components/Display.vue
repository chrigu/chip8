<template>
  <div class="chip8-display">
    <canvas class="display"></canvas>
  </div>
</template>

<script>
export default {
  name: 'Display',
  methods: {
    initDisplay() {
      this.$chip8.initDisplay('display')
    }
  },
  mounted() {
    setTimeout(this.initDisplay, 0)
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

$display-width: 640px;
$display-height: 320px;

.display {
    width: $display-width;
    height: $display-height;
    background-color: #222222;

    @media only screen and (max-width: 650px) {
      width: $display-width/2;
      height: $display-height/2;
    }
}
</style>
