<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

const greetMsg = ref<string>('')
const name = ref<string>('')

async function greet() {
  if (!name.value.trim())
    return
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <div>
    <input v-model="name" placeholder="Enter a name..." border="~ dashed brand-primary" rounded outline-none py-1 px-2 mr-10px @keydown.enter="greet()">
    <button btn @click="greet()">
      Greet
    </button>
  </div>

  <p leading-12>{{ greetMsg }}</p>
</template>
