<template>
  <div class="rom-selection">
    <div class="rom-list">
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
