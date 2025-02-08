<template>
  <div class="container">
    <h1>System Profiling POC</h1>

    <!-- System Architecture Overview -->
    <div class="section architecture">
      <h2>System Architecture</h2>
      <div class="arch-diagram">
        <div class="node client">
          <h3>Client Node</h3>
          <div class="status" :class="{ active: isClientRunning }">
            Status: {{ isClientRunning ? 'Running' : 'Idle' }}
          </div>
        </div>
        <div class="arrow">→</div>
        <div class="node server">
          <h3>Backend Server</h3>
          <div class="endpoints">
            <div>gRPC: [::1]:50051</div>
            <div>HTTP: [::1]:3000</div>
          </div>
          <div class="status" :class="{ active: isServerConnected }">
            Status: {{ isServerConnected ? 'Connected' : 'Checking...' }}
          </div>
        </div>
        <div class="arrow">→</div>
        <div class="node ui">
          <h3>Frontend UI</h3>
          <div class="status active">Active</div>
        </div>
      </div>
    </div>

    <!-- Task Control Panel -->
    <div class="section control-panel">
      <h2>Profile Task Control</h2>
      <div class="task-options">
        <div class="task-group">
          <h3>Available Tasks</h3>
          <div class="task-list">
            <button 
              @click="runTask('cpu')" 
              :disabled="isClientRunning"
              class="task-button"
            >
              CPU Intensive Task
            </button>
            <button 
              @click="runTask('memory')" 
              :disabled="isClientRunning"
              class="task-button"
            >
              Memory Intensive Task
            </button>
            <button 
              @click="runTask('mixed')" 
              :disabled="isClientRunning"
              class="task-button"
            >
              Mixed Workload Task
            </button>
          </div>
        </div>
        <div class="task-info" v-if="currentTask">
          <h3>Current Task</h3>
          <div>Type: {{ currentTask.type }}</div>
          <div>Started: {{ currentTask.startTime }}</div>
          <div>Status: {{ currentTask.status }}</div>
        </div>
      </div>
    </div>

    <!-- Profile Viewer -->
    <div class="section profile-viewer">
      <h2>Profile Visualization</h2>
      <div v-if="currentProfileId" class="flame-container">
        <FlameGraph :profileId="currentProfileId" :key="currentProfileId" />
      </div>
      <div v-else class="placeholder">
        Run a task to generate profile data
      </div>
    </div>

    <!-- Recent Profiles -->
    <div class="section recent-profiles">
      <h2>Recent Profiles</h2>
      <div class="profile-list">
        <div 
          v-for="profile in recentProfiles" 
          :key="profile.id"
          class="profile-item"
          :class="{ active: currentProfileId === profile.id }"
          @click="loadProfile(profile.id)"
        >
          <div class="profile-info">
            <div class="profile-type">{{ profile.taskType }}</div>
            <div class="profile-time">{{ profile.timestamp }}</div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="error" class="error">
      {{ error }}
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import FlameGraph from './components/FlameGraph.vue'

export default {
  components: {
    FlameGraph
  },
  setup() {
    const isClientRunning = ref(false)
    const isServerConnected = ref(false)
    const currentProfileId = ref(null)
    const currentTask = ref(null)
    const error = ref(null)
    const recentProfiles = ref([])

    // Check server connection on mount
    onMounted(async () => {
      try {
        const response = await fetch('http://[::1]:3000/health')
        isServerConnected.value = response.ok
      } catch (e) {
        error.value = 'Server connection failed'
      }
    })

    const runTask = async (taskType) => {
      isClientRunning.value = true
      currentTask.value = {
        type: taskType,
        startTime: new Date().toLocaleTimeString(),
        status: 'Running'
      }

      try {
        const response = await fetch(`http://[::1]:3000/api/tasks/run`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ type: taskType })
        })
        
        if (!response.ok) throw new Error('Task failed to start')
        
        const { profileId } = await response.json()
        currentProfileId.value = profileId
        
        // Add to recent profiles
        recentProfiles.value.unshift({
          id: profileId,
          taskType,
          timestamp: new Date().toLocaleTimeString()
        })

        currentTask.value.status = 'Completed'
      } catch (e) {
        error.value = e.message
        currentTask.value.status = 'Failed'
      } finally {
        isClientRunning.value = false
      }
    }

    const loadProfile = (profileId) => {
      currentProfileId.value = profileId
    }

    return {
      isClientRunning,
      isServerConnected,
      currentProfileId,
      currentTask,
      error,
      recentProfiles,
      runTask,
      loadProfile
    }
  }
}
</script>

<style scoped>
.container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.section {
  margin-bottom: 30px;
  padding: 20px;
  border-radius: 8px;
  background: white;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.architecture {
  background: #f8f9fa;
}

.arch-diagram {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  gap: 20px;
}

.node {
  padding: 15px;
  border-radius: 6px;
  background: white;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  min-width: 200px;
  text-align: center;
}

.arrow {
  font-size: 24px;
  color: #666;
}

.status {
  margin-top: 10px;
  padding: 5px;
  border-radius: 4px;
  background: #f0f0f0;
}

.status.active {
  background: #4caf50;
  color: white;
}

.task-options {
  display: flex;
  gap: 20px;
}

.task-group {
  flex: 1;
}

.task-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.task-button {
  padding: 10px;
  border: none;
  border-radius: 4px;
  background: #2196f3;
  color: white;
  cursor: pointer;
}

.task-button:disabled {
  background: #ccc;
}

.flame-container {
  height: 500px;
  border: 1px solid #eee;
  border-radius: 4px;
}

.placeholder {
  height: 500px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f8f9fa;
  border-radius: 4px;
  color: #666;
}

.profile-list {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.profile-item {
  padding: 10px;
  border-radius: 4px;
  background: #f8f9fa;
  cursor: pointer;
}

.profile-item.active {
  background: #e3f2fd;
}

.error {
  padding: 10px;
  margin-top: 10px;
  background: #ffebee;
  color: #c62828;
  border-radius: 4px;
}
</style>
