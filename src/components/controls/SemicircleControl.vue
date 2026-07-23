<script setup>
  import { Chart as ChartJS, ArcElement, Tooltip } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'
  import { ref, computed } from 'vue'

  // import { useCounterStore } from '../../stores/counter'
  // const counter = useCounterStore()

  const props = defineProps({
    value: {
      type: Number,
      required: true
    }
  });

  ChartJS.register(ArcElement, Tooltip)

  // const speed = ref(counter.valueKw)
  const maxSpeed = 180

  console.log('VALUE', props.value)
  const percent = computed(() => props.value / maxSpeed * 100)

  const chartData = computed(() => ({
    datasets: [
      {
        data: [percent.value, 100 - percent.value],
        backgroundColor: ['#155dfc', '#e5e7eb'],
        borderWidth: 0,
        circumference: 180,
        rotation: 270,
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
    <Doughnut :data="chartData" :options="chartOptions" class="w-[180px] h-[180px]" />
    <div class="text-gray-900 dark:text-white absolute bottom-10 text-4xl font-bold">{{ value }} Kw</div>
  </div>
</template>