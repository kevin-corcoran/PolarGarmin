<template>
  <div id="app">
    <header>
      <h1>Running Activities Chart</h1>
    </header>
    <main>
      <div class="chart-container">
        <canvas ref="chartCanvas"></canvas>
      </div>
      <div class="controls">
        <button @click="switchChartType('bar')">Bar Chart</button>
        <button @click="switchChartType('line')">Line Chart</button>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import Chart from 'chart.js/auto'

const chartCanvas = ref(null)
let chartInstance = null

// Sample running data
const runningData = ref([
  { date: '2025-09-24', distance: 3.8, day: 'Wednesday' },
  { date: '2025-09-25', distance: 0.0, day: 'Thursday' },
  { date: '2025-09-26', distance: 0.0, day: 'Friday' },
  { date: '2025-09-27', distance: 4.7, day: 'Saturday' },
  { date: '2025-09-28', distance: 0.0, day: 'Sunday' },
  { date: '2025-09-29', distance: 5.2, day: 'Monday' },
  { date: '2025-09-30', distance: 0.0, day: 'Tuesday' },
  { date: '2025-10-01', distance: 3.5, day: 'Wednesday' },
  { date: '2025-10-02', distance: 0.0, day: 'Thursday' },
  { date: '2025-10-03', distance: 6.1, day: 'Friday' }
])

const createChart = (type = 'bar') => {
  if (!chartCanvas.value) return
  
  // Destroy existing chart
  if (chartInstance) {
    chartInstance.destroy()
  }

  // Prepare data for chart
  const labels = runningData.value.map(activity => {
    // Format date as "MMM DD" (e.g., "Sep 27")
    const date = new Date(activity.date)
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
  })

  const distances = runningData.value.map(activity => activity.distance)

  const chartConfig = {
    type: type,
    data: {
      labels: labels,
      datasets: [
        {
          label: 'Distance (miles)',
          data: distances,
          backgroundColor: type === 'bar' ? distances.map(distance => 
            distance > 0 ? 'rgba(54, 162, 235, 0.8)' : 'rgba(200, 200, 200, 0.3)'
          ) : 'rgba(54, 162, 235, 0.1)',
          borderColor: 'rgba(54, 162, 235, 1)',
          borderWidth: type === 'bar' ? 1 : 2,
          borderRadius: type === 'bar' ? 2 : 0,
          tension: type === 'line' ? 0.1 : 0,
          pointBackgroundColor: distances.map(distance => 
            distance > 0 ? 'rgba(54, 162, 235, 1)' : 'rgba(200, 200, 200, 0.5)'
          ),
          pointBorderColor: 'white',
          pointBorderWidth: 2,
          pointRadius: distances.map(distance => 
            distance > 0 ? 5 : 3
          ),
          fill: type === 'line'
        }
      ]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        title: {
          display: true,
          text: type === 'bar' ? 'Running Distance by Day' : 'Running Distance Over Time',
          font: {
            size: 16
          }
        },
        tooltip: {
          callbacks: {
            label: function(context) {
              const distance = context.raw
              const day = runningData.value[context.dataIndex].day
              return `${day}: ${distance.toFixed(2)} miles`
            }
          }
        },
        legend: {
          display: false
        }
      },
      scales: {
        x: {
          title: {
            display: true,
            text: 'Date'
          },
          grid: {
            display: false
          }
        },
        y: {
          title: {
            display: true,
            text: 'Distance (miles)'
          },
          beginAtZero: true,
          ticks: {
            callback: function(value) {
              return value + ' mi'
            }
          }
        }
      }
    }
  }

  chartInstance = new Chart(chartCanvas.value, chartConfig)
}

const switchChartType = (type) => {
  createChart(type)
}

onMounted(() => {
  createChart('bar')
})

onUnmounted(() => {
  if (chartInstance) {
    chartInstance.destroy()
  }
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background-color: #f5f5f5;
}

#app {
  max-width: 1000px;
  margin: 0 auto;
  padding: 20px;
}

header {
  text-align: center;
  margin-bottom: 2rem;
}

header h1 {
  color: #333;
  font-size: 2rem;
}

.chart-container {
  position: relative;
  height: 500px;
  background: white;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  margin-bottom: 1rem;
}

.controls {
  text-align: center;
}

button {
  background-color: #4CAF50;
  border: none;
  color: white;
  padding: 10px 20px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #45a049;
}

button:active {
  background-color: #3d8b40;
}
</style>
