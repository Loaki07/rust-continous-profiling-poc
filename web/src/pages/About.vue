<template>
  <div class="about-layout">
    <AppHeader />
    <main class="main-content">
      <div class="container">
        <div class="section overview">
          <h2>Overview</h2>
          <p>A tool for continuous profiling of Rust applications, enabling real-time performance monitoring and analysis through flame graphs.</p>
        </div>

        <div class="section architecture">
          <h2>System Architecture</h2>
          <div class="component-grid">
            <div class="component-card">
              <h3>Frontend UI</h3>
              <ul>
                <li>Vue.js application</li>
                <li>Real-time visualization</li>
                <li>Task control interface</li>
                <li>Flame graph rendering</li>
              </ul>
            </div>
            <div class="component-card">
              <h3>Backend Server</h3>
              <ul>
                <li>Dual-protocol (HTTP/gRPC)</li>
                <li>Profile data processing</li>
                <li>Task coordination</li>
                <li>Data persistence</li>
              </ul>
            </div>
            <div class="component-card">
              <h3>Task Daemon</h3>
              <ul>
                <li>Workload execution</li>
                <li>Profile collection</li>
                <li>pprof integration</li>
                <li>Resource monitoring</li>
              </ul>
            </div>
          </div>
        </div>

        <div class="section features">
          <h2>Key Features</h2>
          <div class="feature-list">
            <div class="feature-item">
              <span class="feature-icon">⚡</span>
              <div class="feature-text">
                <h4>Real-time Performance Profiling</h4>
                <p>Continuous monitoring and analysis of application performance metrics</p>
              </div>
            </div>
            <div class="feature-item">
              <span class="feature-icon">🔄</span>
              <div class="feature-text">
                <h4>Multiple Workload Types</h4>
                <p>Support for CPU, Memory, and Mixed workload profiling</p>
              </div>
            </div>
            <div class="feature-item">
              <span class="feature-icon">📊</span>
              <div class="feature-text">
                <h4>Flame Graph Visualization</h4>
                <p>Interactive visualization of call stack performance data</p>
              </div>
            </div>
            <div class="feature-item">
              <span class="feature-icon">🔌</span>
              <div class="feature-text">
                <h4>Distributed Architecture</h4>
                <p>Scalable system with separate frontend, backend, and daemon components</p>
              </div>
            </div>
          </div>
        </div>

        <div class="section tech-stack">
          <h2>Technology Stack</h2>
          <div class="stack-grid">
            <div class="stack-column">
              <h3>Backend</h3>
              <ul>
                <li>Rust</li>
                <li>Tonic (gRPC)</li>
                <li>Actix-web</li>
                <li>pprof</li>
              </ul>
            </div>
            <div class="stack-column">
              <h3>Frontend</h3>
              <ul>
                <li>Vue.js</li>
                <li>TypeScript</li>
                <li>ECharts</li>
              </ul>
            </div>
            <div class="stack-column">
              <h3>Infrastructure</h3>
              <ul>
                <li>Docker</li>
                <li>Kubernetes</li>
                <li>Nginx</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import AppHeader from '../components/AppHeader.vue'
</script>

<style scoped>
.about-layout {
  min-height: 100vh;
  background: #f8f9fa;
}

.main-content {
  padding-top: 64px;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 32px;
}

.section {
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  padding: 32px;
  margin-bottom: 32px;
}

h2 {
  margin: 0 0 24px 0;
  font-size: 24px;
  font-weight: 500;
  color: #2c3e50;
}

.overview p {
  font-size: 16px;
  line-height: 1.6;
  color: #444;
}

.component-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}

.component-card {
  background: #f8f9fa;
  padding: 24px;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
}

.component-card h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  color: #2c3e50;
}

.component-card ul {
  margin: 0;
  padding-left: 20px;
  color: #444;
}

.component-card li {
  margin-bottom: 8px;
}

.feature-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;
}

.feature-item {
  display: flex;
  gap: 16px;
  align-items: flex-start;
}

.feature-icon {
  font-size: 24px;
  background: #e3f2fd;
  padding: 12px;
  border-radius: 8px;
}

.feature-text h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: #2c3e50;
}

.feature-text p {
  margin: 0;
  color: #666;
  line-height: 1.5;
}

.stack-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}

.stack-column h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  color: #2c3e50;
}

.stack-column ul {
  margin: 0;
  padding-left: 20px;
  color: #444;
}

.stack-column li {
  margin-bottom: 8px;
}
</style> 