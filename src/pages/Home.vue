<script setup lang="ts">
const { x, y } = useMouse()

const router = useRouter()
function toLogin() {
  router.push({
    name: 'Login',
  })
}
</script>

<template>
  <div flex="c col">
    <h1 color-brand-primary my-20px text="28px" flex-c>
      Welcome to Tauri!
      <div ml-4 inline-block i-logos:tauri />
    </h1>
    <button btn @click="toLogin">
      Login
    </button>
    <div border p-4 my-3>
      Mouse ({{ x }}, {{ y }}) is over the window.
    </div>
    <Greet />
  </div>
</template>

<style scoped lang="scss"></style>
