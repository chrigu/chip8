<template>
  <div  class="app">
    <header>
      <h1>Chip8 Emulator</h1>
    </header>
    <section class="emulator">
      <router-view/>
    </section>
    <footer class="footer">
      <div class="content has-text-centered">
        <p>
          some footer
        </p>
      </div>
    </footer>
  </div>
</template>

<style lang="scss">
.app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: rgb(76, 179, 91);
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.emulator {
  flex: 1;
}

</style>
