<template>
  <div class="rom-uploader">
    <!--UPLOAD-->
    <form enctype="multipart/form-data" novalidate v-if="isInitial || isSaving">
      <h1>Upload images</h1>
      <div class="dropbox">
        <input type="file" name="rom" :disabled="isSaving" @change="filesChange($event.target.files)"
            class="input-file">
          <p v-if="isInitial">
            Drag your file(s) here to begin<br> or click to browse
          </p>
          <p v-if="isSaving">
            Uploading {{ fileCount }} files...
          </p>
      </div>
    </form>
    <!--SUCCESS-->
    <div v-if="isSuccess">
      <p>
        <a href="javascript:void(0)" @click="reset()">Upload again</a>
      </p>
      <output id="list">{{romName}}</output>
    </div>
  </div>
</template>

<script>
// https://github.com/chybie/file-upload-vue/blob/master/src/App.vue
import { mapActions } from 'vuex'

const STATUS_INITIAL = 0, STATUS_SAVING = 1, STATUS_SUCCESS = 2, STATUS_FAILED = 3;

export default {
  name: 'RomUploader',
  data () {
    return {
      romName: '',
      currentStatus: null,
    }
  },
  computed: {
    isInitial() {
      return this.currentStatus === STATUS_INITIAL;
    },
    isSaving() {
      return this.currentStatus === STATUS_SAVING;
    },
    isSuccess() {
      return this.currentStatus === STATUS_SUCCESS;
    },
    isFailed() {
      return this.currentStatus === STATUS_FAILED;
    }
  },
  methods: {
    ...mapActions(['setRom']),
    filesChange(fileList) {
      const that = this;

      // files is a FileList of File objects. List some properties.
      let output = [];
      for (let i = 0, f; (f = fileList[i]); i++) {
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
      this.setRom(fileList[0])
      this.currentStatus = STATUS_SUCCESS
    },
    reset() {
      // reset form to initial state
      this.currentStatus = STATUS_INITIAL;
      this.uploadedFiles = [];
      this.uploadError = null;
    },
  },
  mounted() {
    this.reset();
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
  .dropbox {
    outline: 2px dashed rgb(76, 179, 91); /* the dash box */
    outline-offset: -10px;
    color: rgb(76, 179, 91);
    padding: 10px 10px;
    min-height: 200px; /* minimum height */
    position: relative;
    cursor: pointer;
    background-image: url(/floppy.svg);
  }

  .input-file {
    opacity: 0; /* invisible but it's there! */
    width: 100%;
    height: 200px;
    position: absolute;
    cursor: pointer;
  }

  .dropbox:hover {
    background: lightblue; /* when mouse over to the drop zone, change color */
  }

  .dropbox p {
    font-size: 1.2em;
    text-align: center;
    padding: 50px 0;
  }
</style>
