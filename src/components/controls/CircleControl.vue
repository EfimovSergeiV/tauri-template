<script setup>
  import { Chart as ChartJS, ArcElement, Tooltip } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'
  import { ref, computed } from 'vue'

  import { useCounterStore } from '../../stores/counter'
  const counter = useCounterStore()

  ChartJS.register(ArcElement, Tooltip)

  const speed = ref(counter.valueKw)
  const maxSpeed = 180

  const percent = computed(() => counter.valueKw / maxSpeed * 100)

  const chartData = computed(() => ({
    datasets: [
      {
        data: [percent.value, 100 - percent.value],
        backgroundColor: ['#155dfc', '#e5e7eb'],
        borderWidth: 0,
        circumference: 360,
        rotation: 180,
        cutout: '95%'
      }
    ]
  }))

  const chartOptions = {
    responsive: true,
    plugins: {
      tooltip: { enabled: false },
    }
  }
</script>


<template>
  <div class="flex flex-col items-center relative select-none">

    <Doughnut :data="chartData" :options="chartOptions" class="w-62.5 h-62.5" />
    <div class="text-gray-900 dark:text-white absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-4xl font-bold">{{ counter.valueKw }} Kw</div>

  </div>
</template>