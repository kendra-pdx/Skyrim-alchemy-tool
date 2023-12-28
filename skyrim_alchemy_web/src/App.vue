<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <header>
    <h1>Skyrim Alchemy</h1>
  </header>
  <main>
    <RouterView />
    <div>
      <a href="https://github.com/kendra-pdx/skyrim-alchemy-tool">Source</a>
    </div>
  </main>
</template>

<style scoped>
main, header {
  margin: 1em;
}
</style>
