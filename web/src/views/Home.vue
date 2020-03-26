<template>
  <div class="home">
    <RomUploader />
    <Display />
  </div>
</template>

<script>
// @ is an alias to /src
import Display from '@/components/Display.vue'
import RomUploader from '@/components/RomUploader.vue'

export default {
  name: 'Home',
  components: {
    Display,
    RomUploader
  },
  mounted: function() {
    const that = this;
    console.log(that)
    window.addEventListener('keyup', function(event) {
      that.$chip8.keyup(event)
    });

    window.addEventListener('keydown', function(event) {
      that.$chip8.keydown(event)
    });
  }
}
</script>
