<template>
  <div class="home">
    <div class="columns">
      <div class="home__main main column is-full" :class="{'is-three-fifths': debugMode}">
        <div class="main__actions actions">
          <RomSelection class="actions__rom" />
          <button class="button is-hidden-touch" @click="toggleDebugMode" :class="{'is-warning': debugMode, 'is-success': !debugMode}">{{debugButtonText}}</button>
        </div>
        <Display class="main__display" />
        <Keyboard />
      </div>
      <div class="home__debug column" :class="{'home__debug__visible': debugMode}">
        <Debug />
      </div>
    </div>
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
  debugButtonText() {
    return this.debugMode ? 'Debug mode off' : 'Debug mode on'
  }
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

  &__debug {
    &--visible {
      width: 100%;
    }
  }
}

.main {
  &__actions {
    display: flex;
    justify-content: center;
  }

  &__display {
    margin-bottom: 2rem;
  }
}

.actions {
  &__rom {
    margin-right: 1rem !important;
  }
}

</style>
