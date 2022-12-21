<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import type { Account, CurrencyType } from './types/core'

const account = ref<Account | null>(null)

async function loadData() {
  const acc = await invoke<Account>('get_account')
  account.value = acc
}

async function consumCurrency(ty: CurrencyType, amount: number) {
  await invoke('consum_currency', { ty, amount })
  loadData()
}

onMounted(() => {
  loadData()
})
</script>

<template>
  <div flex="c col" h-100vh bg-white>
    {{ account }}
    <button btn @click="consumCurrency('gold', 50)">
      gold -50
    </button>
    <router-view />
  </div>
</template>

<style scoped>

</style>
