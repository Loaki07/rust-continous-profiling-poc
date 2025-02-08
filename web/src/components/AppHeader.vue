<template>
  <header class="app-header">
    <div class="header-content">
      <div class="app-title">
        <h1>Rust Continuous Profiling Tool</h1>
      </div>
      <nav class="main-nav">
        <router-link to="/" :class="{ active: currentRoute === '/' }">Dashboard</router-link>
        <router-link to="/about" :class="{ active: currentRoute === '/about' }">About</router-link>
      </nav>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const currentRoute = computed(() => route.path)
</script>

<style scoped>
.app-header {
  background: #fff;
  border-bottom: 1px solid #e0e0e0;
  padding: 0;
  height: 64px;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 32px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.app-title h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 500;
  color: #2c3e50;
}

.main-nav {
  display: flex;
  gap: 32px;
}

.main-nav a {
  color: #666;
  text-decoration: none;
  font-size: 15px;
  font-weight: 500;
  padding: 8px 0;
  border-bottom: 2px solid transparent;
  transition: all 0.2s ease;
}

.main-nav a:hover {
  color: #2196f3;
}

.main-nav a.active {
  color: #2196f3;
  border-bottom-color: #2196f3;
}
</style> 