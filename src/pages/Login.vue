<script setup lang="ts">
const router = useRouter()

function goHome() {
  router.push({
    name: 'Home',
  })
}
</script>

<template>
  <div flex="c col">
    <div my-3>
      login page
    </div>
    <button btn @click="goHome()">
      back home
    </button>
  </div>
</template>

<style scoped lang="scss"></style>
