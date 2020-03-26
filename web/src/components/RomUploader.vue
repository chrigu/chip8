<template>
  <div class="rom-uploader">
    <input type="file" name="rom" @change="handleFileSelect"/>
    <output id="list">{{romName}}</output>
  </div>
</template>

<script>
export default {
  name: 'RomUploader',
  data () {
    return {
      romName: ''
    }
  },
  methods: {
    handleFileSelect(evt) {
      let files = evt.target.files; // FileList object

      // files is a FileList of File objects. List some properties.
      var output = [];
      for (var i = 0, f; (f = files[i]); i++) {
        output.push(
          escape(f.name),
          f.type || "n/a",
          ") - ",
          f.size,
          " bytes, last modified: ",
          f.lastModifiedDate ? f.lastModifiedDate.toLocaleDateString() : "n/a",
        );
      }

      this.romName = output;

      const buffer = new ArrayBuffer(3584);
      const u8Buffer = new Uint8Array(buffer);

      var reader = new FileReader();
      reader.onload = function(theFile) {
        this.$chip8(theFile, reader.result)
      }
    }
  },
  mounted() {
    this.$chip8.initDisplay('display')
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>
