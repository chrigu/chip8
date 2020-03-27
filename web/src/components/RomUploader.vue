<template>
  <div class="rom-uploader">
    <input type="file" name="rom" @change="handleFileSelect"/>
    <output id="list">{{romName}}</output>
  </div>
</template>

<script>

import { mapActions } from 'vuex'

export default {
  name: 'RomUploader',
  data () {
    return {
      romName: ''
    }
  },
  methods: {
    ...mapActions(['setRom']),
    handleFileSelect(evt) {
      const that = this;
      let files = evt.target.files; // FileList object

      // files is a FileList of File objects. List some properties.
      let output = [];
      for (let i = 0, f; (f = files[i]); i++) {
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
      this.setRom(files[0])
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>
