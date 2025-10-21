<script setup>

  import { ref, onMounted, onUnmounted } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  import Chart from 'chart.js/auto'
  const ctx = document.getElementById('myChart');
  new Chart(ctx, {
    type: 'bar',
    data: {
      labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
      datasets: [{
        label: '# of Votes',
        data: [12, 19, 3, 5, 2, 3],
        borderWidth: 1
      }]
    },
    options: {
      scales: {
        y: {
          beginAtZero: true
        }
      }
    }
  })
</script>

<style scoped>
.chart-container {
  position: relative;
  height: 400px;
  margin-bottom: 2rem;
}

.data-table {
  margin-top: 2rem;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

th {
  background-color: #f2f2f2;
}

.error {
  color: red;
}
</style>
