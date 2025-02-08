<template>
  <div class="about-container">
    <h1>Rust Continuous Profiling Tool</h1>
    
    <section class="overview">
      <h2>Overview</h2>
      <p>
        A tool for continuous profiling of Rust applications, enabling real-time
        performance monitoring and analysis through flame graphs.
      </p>
    </section>

    <section class="architecture">
      <h2>System Architecture</h2>
      <div class="architecture-diagram">
        <!-- You can add a more detailed SVG diagram here -->
        <div class="component frontend">
          <h3>Frontend UI</h3>
          <ul>
            <li>Vue.js application</li>
            <li>Real-time visualization</li>
            <li>Task control interface</li>
            <li>Flame graph rendering</li>
          </ul>
        </div>
        
        <div class="component server">
          <h3>Backend Server</h3>
          <ul>
            <li>Dual-protocol (HTTP/gRPC)</li>
            <li>Profile data processing</li>
            <li>Task coordination</li>
            <li>Data persistence</li>
          </ul>
        </div>
        
        <div class="component daemon">
          <h3>Task Daemon</h3>
          <ul>
            <li>Workload execution</li>
            <li>Profile collection</li>
            <li>pprof integration</li>
            <li>Resource monitoring</li>
          </ul>
        </div>
      </div>
    </section>

    <section class="features">
      <h2>Key Features</h2>
      <ul>
        <li>Real-time performance profiling</li>
        <li>Multiple workload types (CPU, Memory, Mixed)</li>
        <li>Flame graph visualization</li>
        <li>Distributed architecture</li>
        <li>Container-ready deployment</li>
        <li>Kubernetes integration</li>
      </ul>
    </section>

    <section class="tech-stack">
      <h2>Technology Stack</h2>
      <div class="tech-categories">
        <div>
          <h3>Backend</h3>
          <ul>
            <li>Rust</li>
            <li>Tonic (gRPC)</li>
            <li>Actix-web</li>
            <li>pprof</li>
          </ul>
        </div>
        <div>
          <h3>Frontend</h3>
          <ul>
            <li>Vue.js</li>
            <li>TypeScript</li>
            <li>ECharts</li>
          </ul>
        </div>
        <div>
          <h3>Infrastructure</h3>
          <ul>
            <li>Docker</li>
            <li>Kubernetes</li>
            <li>Nginx</li>
          </ul>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.about-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

section {
  margin-bottom: 3rem;
}

.architecture-diagram {
  display: flex;
  justify-content: space-between;
  gap: 2rem;
  margin: 2rem 0;
}

.component {
  flex: 1;
  padding: 1.5rem;
  border-radius: 8px;
  background: var(--el-bg-color-overlay);
  box-shadow: var(--el-box-shadow-light);
}

.tech-categories {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 2rem;
  margin-top: 1rem;
}

h1 {
  font-size: 2.5rem;
  margin-bottom: 2rem;
  color: var(--el-text-color-primary);
}

h2 {
  font-size: 1.8rem;
  margin-bottom: 1rem;
  color: var(--el-text-color-primary);
}

h3 {
  font-size: 1.4rem;
  margin-bottom: 0.8rem;
  color: var(--el-text-color-regular);
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  margin: 0.5rem 0;
  color: var(--el-text-color-regular);
}

p {
  line-height: 1.6;
  color: var(--el-text-color-regular);
}
</style> 