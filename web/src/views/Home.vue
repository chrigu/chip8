<template>
  <div class="home">
    <button class="debug-button" @click="toggleDebugMode">Debug mode</button>
    <div class="columns">
      <div class="home__main column is-full" :class="{'is-two-thirds': debugMode}">
        <RomSelection />
        <Display />
      </div>
      <div class="home__debug column" :class="{'is-one-thirds': debugMode}">
        <Debug />
      </div>
    </div>
    <Keyboard />
  </div>
</template>

<script>
// @ is an alias to /src
import Display from '@/components/Display.vue'
import RomSelection from '@/components/RomSelection.vue'
import Debug from '@/components/Debug.vue'
import Keyboard from '@/components/Keyboard.vue'

import { mapActions, mapGetters } from 'vuex'

export default {
  name: 'Home',
  components: {
    Display,
    RomSelection,
    Debug,
    Keyboard
  },
  computed: {
  ...mapGetters(['debugMode']),
  },
  methods: {
    ...mapActions(['debugModeOn', 'debugModeOff', 'keydown', 'keyup']),
    toggleDebugMode() {
      this.debugMode ? this.debugModeOff() : this.debugModeOn(); // move to store
    }
  },
  mounted: function() {
    const that = this;
    window.addEventListener('keyup', function(event) {
      that.keyup(event.key)
    });

    window.addEventListener('keydown', function(event) {
      that.keydown(event.key)
    });
  }
}
</script>

<style lang="scss">
.debug-button {
  margin-bottom: 1em;
}

.home {
  &__main, &__debug {
    transition: width 0.5s ease-in-out;;
  }
}

</style>
