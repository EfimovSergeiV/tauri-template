<script setup>
  import { useCounterStore } from '../stores/counter'
  const counter = useCounterStore()


</script>


<template>
  <div class="">
    this is footer

    <div class="py-8 flex gap-6">
      <button class="bg-red-400 px-8 py-4 text-white" @click="counter.decKw" > Уменьшить </button>
      <button class="bg-blue-400 px-8 py-4 text-white" @click="counter.incKw" > Увеличить </button>

    </div>
  </div>
</template>