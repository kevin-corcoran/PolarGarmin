<script setup>
import { ref, onMounted } from 'vue'
// invoke doesnt work in browser. Seems silly. How are you supposed to
// communicate with the backend for a web app?
import { invoke } from '@tauri-apps/api/core'
// import tauriapi from '@tauri-apps/api';
// const { invoke } = tauriapi.tauri;
// app.withGlobalTauri = true in tauri.conf.json
// const invoke = window.__TAURI__.core.invoke;
// const invoke = window.__TAURI_INTERNALS__.invoke;

const runningData = ref([])
const loading = ref(false)
const error = ref('')

const fetchData = async () => {
  loading.value = true
  error.value = ''
  try {
    runningData.value = await invoke('fetch_running_data')
  } catch (err) {
    error.value = err.toString()
  } finally {
    loading.value = false
  }
}


onMounted(() => {
  fetchData()
})
</script>

<template>

  <div>
    <h2>Running Activities</h2>
    <div v-if="loading">Loading...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else>
      <table>
        <thead>
          <tr>
            <th>Date</th>
            <th>Day</th>
            <th>Month</th>
            <th>Time</th>
            <th>Distance (km)</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="activity in runningData" :key="activity.date">
            <td>{{ activity.date }}</td>
            <td>{{ activity.day }}</td>
            <td>{{ activity.month }}</td>
            <td>{{ activity.start_time }}</td>
            <td>{{ activity.distance.toFixed(2) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>


<style scoped>
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

