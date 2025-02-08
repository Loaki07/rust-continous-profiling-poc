<template>
  <div class="dashboard-layout">
    <AppHeader />
    <main class="main-content">
      <div class="container">
        <div class="top-bar">
          <div class="status-bar">
            <div class="status-nodes">
              <div class="node client">
                <span class="node-label">Task Node</span>
                <div class="status" :class="{ active: isClientRunning }">
                  {{ isClientRunning ? 'Running' : 'Idle' }}
                </div>
              </div>
              <div class="connector"></div>
              <div class="node server">
                <span class="node-label">Backend Server</span>
                <div class="status" :class="{ active: isServerConnected }">
                  {{ isServerConnected ? 'Connected' : 'Checking...' }}
                </div>
              </div>
              <div class="connector"></div>
              <div class="node ui">
                <span class="node-label">Frontend UI</span>
                <div class="status active">Active</div>
              </div>
            </div>
          </div>
          <div class="action-buttons">
            <button 
              class="action-button primary"
              @click="runTask('cpu')"
              :disabled="isClientRunning"
            >
              <span class="icon">⚡</span>
              Run CPU Profile
            </button>
            <button 
              class="action-button primary"
              @click="runTask('memory')"
              :disabled="isClientRunning"
            >
              <span class="icon">💾</span>
              Run Memory Profile
            </button>
            <button 
              class="action-button primary"
              @click="runTask('mixed')"
              :disabled="isClientRunning"
            >
              <span class="icon">🔄</span>
              Run Mixed Profile
            </button>
          </div>
        </div>

        <div class="recent-profiles-bar">
          <span class="recent-label">Recent Profiles:</span>
          <div class="recent-chips">
            <div 
              v-for="profile in recentProfiles.slice(0, 5)" 
              :key="profile.id"
              class="profile-chip"
              :class="{ active: currentProfileId === profile.id }"
              @click="loadProfile(profile.id)"
            >
              <span class="chip-icon">
                {{ profile.taskType === 'cpu' ? '⚡' : 
                   profile.taskType === 'memory' ? '💾' : '🔄' }}
              </span>
              <span class="chip-time">{{ profile.timestamp }}</span>
            </div>
          </div>
        </div>

        <div class="graphs">
          <div class="graph-panel">
            <h3>Flame Graph View</h3>
            <FlameGraph :profile-data="profileData" view="flame" />
          </div>
          
          <div class="graph-panel">
            <h3>Sunburst View</h3>
            <FlameGraph :profile-data="profileData" view="sunburst" />
          </div>
        </div>

        <div v-if="error" class="error">
          {{ error }}
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import FlameGraph from './FlameGraph.vue'
import AppHeader from './AppHeader.vue'

const isClientRunning = ref(false)
const isServerConnected = ref(false)
const currentProfileId = ref(null)
const currentTask = ref(null)
const error = ref(null)
const recentProfiles = ref([])
const profileData = ref(null)

// Check server connection on mount
onMounted(async () => {
  try {
    const response = await fetch('http://[::1]:3000/health')
    isServerConnected.value = response.ok
  } catch (e) {
    error.value = 'Server connection failed'
  }
})

async function fetchProfileData(profileId: string) {
  try {
    const response = await fetch(`http://[::1]:3000/api/profiles/${profileId}`)
    if (!response.ok) throw new Error('Failed to fetch profile')
    const data = await response.json()
    
    console.log('Raw profile data:', data)
    
    // Calculate total value from all nodes
    const calculateTotal = (node: any): number => {
      if (!node.children?.length) return node.value || 0
      return Math.max(
        node.value || 0,
        node.children.reduce((sum: number, child: any) => sum + calculateTotal(child), 0)
      )
    }
    
    const totalValue = calculateTotal(data)
    console.log('Total value:', totalValue)
    
    // Transform the data into a tree structure if needed
    profileData.value = {
      name: 'Total',
      value: totalValue,
      children: data.children?.map(child => ({
        ...child,
        value: child.value || calculateTotal(child)
      })) || []
    }
    
    console.log('Transformed profile data:', profileData.value)
    console.log('Children count:', profileData.value.children.length)
    console.log('First child:', profileData.value.children[0])
    
    // Validate the data
    if (!profileData.value.value) {
      console.warn('No value in profile data')
    }
    if (!profileData.value.children?.length) {
      console.warn('No children in profile data')
    }
  } catch (e) {
    error.value = 'Failed to load profile data'
    console.error('Profile data fetch error:', e)
  }
}

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
    
    // Fetch and transform profile data
    await fetchProfileData(profileId)
    
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

const loadProfile = async (profileId) => {
  currentProfileId.value = profileId
  await fetchProfileData(profileId)
}
</script>

<style scoped>
.dashboard-layout {
  min-height: 100vh;
  background: #f8f9fa;
}

.main-content {
  padding-top: 56px;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 12px;
}

.top-bar {
  background: #fff;
  padding: 12px;
  margin-bottom: 16px;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status-bar {
  padding: 8px;
  display: flex;
  align-items: center;
}

.status-nodes {
  display: flex;
  align-items: center;
  gap: 16px;
}

.node {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  border-radius: 4px;
  background: #fff;
  border: 1px solid #e0e0e0;
}

.node-label {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.connector {
  flex: 1;
  height: 2px;
  background: #e0e0e0;
  margin: 0 8px;
  position: relative;
}

.connector::after {
  content: '→';
  position: absolute;
  right: -8px;
  top: -10px;
  color: #666;
}

.status {
  padding: 4px 8px;
  border-radius: 4px;
  background: #f0f0f0;
  font-size: 13px;
}

.status.active {
  background: #4caf50;
  color: white;
}

.action-buttons {
  display: flex;
  gap: 8px;
}

.action-button {
  padding: 8px 16px;
  font-size: 14px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  background: #f5f5f5;
  color: #333;
  transition: all 0.2s ease;
  font-weight: 500;
}

.action-button.primary {
  background: #2196f3;
  color: white;
}

.action-button:hover {
  background: #1976d2;
}

.action-button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.action-button .icon {
  font-size: 16px;
}

.recent-profiles-bar {
  background: #fff;
  padding: 8px 12px;
  margin-bottom: 16px;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  display: flex;
  align-items: center;
  gap: 12px;
}

.recent-label {
  font-size: 14px;
  font-weight: 500;
  color: #333;
  white-space: nowrap;
}

.recent-chips {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.profile-chip {
  padding: 3px 8px;
  font-size: 13px;
  color: #333;
}

.profile-chip:hover {
  background: #e3f2fd;
  border-color: #2196f3;
}

.profile-chip.active {
  background: #e3f2fd;
  border-color: #2196f3;
  color: #1976d2;
}

.chip-icon {
  font-size: 14px;
}

.chip-time {
  color: #666;
}

.graphs {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-top: 16px;
  height: calc(100vh - 200px);  /* Leave space for header and top bars */
}

.graph-panel {
  background: #fff;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
  padding: 16px;
  flex: 1;  /* Take equal space */
  min-height: 0;  /* Allow flex container to shrink */
  display: flex;
  flex-direction: column;
}

.graph-panel h3 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 500;
  color: #333;
  flex: 0 0 auto;  /* Don't allow header to shrink */
}

/* Add a wrapper for the FlameGraph to handle sizing */
.graph-panel :deep(.flame-container) {
  flex: 1;
  min-height: 0;
}

/* Update chart text styles in FlameGraph component */
.graph-panel :deep(.chart) {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  font-size: 14px;
}

.error {
  position: fixed;
  bottom: 20px;
  right: 20px;
  padding: 12px 16px;
  background: #ffebee;
  color: #c62828;
  border-radius: 4px;
  z-index: 1000;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
</style>
