<script setup lang="ts">
import { ExchangeItem } from '../types/core';

const exchangeList = ref<ExchangeItem[]>([
  {
    id: 0,
    name: '朴素的御魂礼盒',
    price: 50,
    count: 30,
  },
  {
    id: 1,
    name: '华丽的御魂礼盒',
    price: 50,
    count: -1,
  },
  {
    id: 2,
    name: '首领的御魂礼盒',
    price: 50,
    count: 50,
  },
  {
    id: 3,
    name: '海汐的御魂礼盒',
    price: 200,
    count: 30,
  }
])
</script>

<template>
  <div flex="c col">
    <div flex="c">
      <div v-for="item in exchangeList" :key="item.id">
        <div>
          本周剩余数量{{ item.count }}个
        </div>
        <span>{{ item.name }}</span>
        <div>
          <span>{{ 0 }}/{{ item.price }}</span>
          <div btn>兑换</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">

</style>
