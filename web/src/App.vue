<template>
  <div  class="app">
    <header>
      <h1 class="app__title">Chip8 Emulator</h1>
    </header>
    <section class="emulator">
      <router-view/>
    </section>
    <footer class="footer">
      <div class="content has-text-centered">
        <p>
          <a class="footer__link" href="https://github.com/chrigu/chip8">Fork me on Github</a>
        </p>
        <p>
          <a class="footer__link" href="https://github.com/dmatlack/chip8/tree/master/roms">ROMs</a>
        </p>
      </div>
    </footer>
  </div>
</template>

<style lang="scss">
.app {
  font-family: 'vt323', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: rgb(76, 179, 91);
  min-height: 100vh;
  display: flex;
  flex-direction: column;

  &__title {
    font-family: 'bitwise', Helvetica, Arial, sans-serif;
    font-size: 32px;
    margin-bottom: 2rem;
  }
}

.emulator {
  flex: 1;
}

.footer {
  background-color: rgb(76, 179, 91) !important;
  &__link {
    color: #222222;

    &:hover {
      color: #222222;
      text-decoration: underline;
    }
  }
}

</style>
