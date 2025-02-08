<template>
  <div class="container">
    <h1>Profile Flame Graph</h1>
    <div class="input-container">
      <input 
        v-model="inputProfileId" 
        placeholder="Enter Profile ID"
        class="profile-input"
      />
      <button @click="loadProfile" class="load-button">Load Profile</button>
    </div>
    <div v-if="currentProfileId" class="flame-container">
      <FlameGraph :profileId="currentProfileId" :key="currentProfileId" />
    </div>
    <div v-if="error" class="error">
      {{ error }}
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import FlameGraph from './components/FlameGraph.vue'

export default {
  components: {
    FlameGraph
  },
  setup() {
    const inputProfileId = ref('')
    const currentProfileId = ref(null)
    const error = ref(null)

    const loadProfile = async () => {
      if (!inputProfileId.value) {
        error.value = 'Please enter a Profile ID'
        return
      }
      
      error.value = null
      try {
        // Test if profile exists before showing flame graph
        const response = await fetch(`http://[::1]:3000/api/profiles/${inputProfileId.value}`)
        if (!response.ok) {
          throw new Error(`Profile not found: ${inputProfileId.value}`)
        }
        currentProfileId.value = inputProfileId.value
      } catch (e) {
        error.value = e.message
        currentProfileId.value = null
      }
    }

    return {
      inputProfileId,
      currentProfileId,
      error,
      loadProfile
    }
  }
}
</script>

<style>
.container {
  padding: 20px;
}

.input-container {
  margin: 20px 0;
  display: flex;
  gap: 10px;
}

.profile-input {
  padding: 8px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 4px;
  width: 300px;
}

.load-button {
  padding: 8px 16px;
  font-size: 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.load-button:hover {
  background-color: #45a049;
}

.flame-container {
  width: 100%;
  height: 600px;
  border: 1px solid #ccc;
  margin: 20px 0;
}

.error {
  color: red;
  margin: 10px 0;
}
</style>
