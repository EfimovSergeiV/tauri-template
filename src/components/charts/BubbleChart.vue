<script setup>
  import { ref, onMounted, onUnmounted } from 'vue'
  import { Bubble } from 'vue-chartjs'
  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    PointElement,
    LinearScale
  } from 'chart.js'

  ChartJS.register(Title, Tooltip, Legend, PointElement, LinearScale)


  const chartData = ref({
    datasets: [
      {
        label: 'Label 1',
        data: [
          { x: 20, y: 30, r: 15 },
          { x: 40, y: 10, r: 10 },
          { x: 80, y: 80, r: 10 },
          { x: 15, y: 25, r: 8 },
          { x: 35, y: 5, r: 12 },
          { x: 25, y: 35, r: 7 }
        ],
        backgroundColor: 'rgba(255, 99, 132, 0.5)'
      },
      {
        label: 'Label 2',
        data: [
          { x: 12, y: 30, r: 15 },
          { x: 20, y: 20, r: 10 },
          { x: 30, y: 40, r: 10 },
          { x: 50, y: 60, r: 8 },
          { x: 70, y: 70, r: 12 },
          { x: 90, y: 90, r: 7 }
        ],
        backgroundColor: 'rgba(54, 162, 235, 0.5)'
      },
    ]
  })

  const chartOptions = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top'
      },
      title: {
        display: false,
        text: 'Пример Bubble Chart'
      }
    }
  }


  const chartRef = ref(null)

  const wsState = ref('')

  const wsUrl = `wss://api.meinewelt.ru/ws`
  let socket = null

  function connectWebSocket() {
    socket = new WebSocket(wsUrl)

    socket.onopen = () => {
      wsState.value = 'connected'
    }

    socket.onmessage = (event) => {
      try {
        const newData = JSON.parse(event.data)

        if (Array.isArray(newData) && newData[0]?.data) {
          const chartInstance = chartRef.value?.chart
          if (chartInstance) {
            chartInstance.data.datasets = newData
            chartInstance.update()
          }
        }
      } catch (e) {
        console.error('Ошибка парсинга данных', e)
      }
    }

    socket.onclose = () => {
      console.log('Соединение закрыто, переподключение...')
      setTimeout(connectWebSocket, 1000)
    }
  }

  setInterval(() => {
    if (socket?.readyState === WebSocket.OPEN) {
      socket.send(JSON.stringify({ chart: 'bubble' }))
    }
  }, 3000)

  onMounted(connectWebSocket)
  onUnmounted(() => socket?.close())

</script>


<template>  
  <div class="">

    <div class="border-b">
      <p class="text-black/80 text-sm">Bubble chart</p>
    </div>
    <div class="">
      <Bubble ref="chartRef" :data="chartData" :options="chartOptions" />
    </div>

  </div>
</template>