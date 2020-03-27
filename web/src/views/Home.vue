<template>
  <div class="home">
    <RomUploader />
    <Display />
    <Debug />
  </div>
</template>

<script>
// @ is an alias to /src
import Display from '@/components/Display.vue'
import RomUploader from '@/components/RomUploader.vue'
import Debug from '@/components/Debug.vue'

export default {
  name: 'Home',
  components: {
    Display,
    RomUploader,
    Debug
  },
  methods: {
    mapKey(keyCode) {
      let chipKeyCode = -1;
      switch (keyCode) {
        case 49: // 1
          chipKeyCode = 1;
          break;
        case 50: // 2
          chipKeyCode = 2;
          break;
        case 51: // 3
          chipKeyCode = 3;
          break;
        case 81: // q
          chipKeyCode = 4;
          break;
        case 87: // w
          chipKeyCode = 5;
          break;
        case 69: // e
          chipKeyCode = 6;
          break;
        case 65: // a
          chipKeyCode = 7;
          break;
        case 83: // s
          chipKeyCode = 8;
          break;
        case 68: // d
          chipKeyCode = 9;
          break;
          case 89: // y
          chipKeyCode = 10;
          break;
        case 88: // x
          chipKeyCode = 11;
          break;
        case 67: // c
          chipKeyCode = 12;
          break;
        default:
          chipKeyCode = -1;
      }
      return chipKeyCode;
    }
  },
  mounted: function() {
    const that = this;
    window.addEventListener('keyup', function(event) {
      let keyCode = that.mapKey(event.keyCode);
      if (keyCode > 0) {
        that.$chip8.keyup(keyCode)
      }
    });

    window.addEventListener('keydown', function(event) {
      let keyCode = that.mapKey(event.keyCode);
      if (keyCode > 0) {
        that.$chip8.keydown(keyCode)
      }
    });
  }
}
</script>
