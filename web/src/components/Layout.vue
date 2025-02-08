<template>
  <el-container>
    <el-header>
      <nav>
        <router-link to="/">Dashboard</router-link>
        <router-link to="/about">About</router-link>
      </nav>
    </el-header>
    <el-main>
      <router-view></router-view>
    </el-main>
  </el-container>
</template>

<style scoped>
nav {
  padding: 1rem 0;
}

nav a {
  margin-right: 1rem;
  color: var(--el-text-color-regular);
  text-decoration: none;
}

nav a.router-link-active {
  color: var(--el-color-primary);
  font-weight: bold;
}
</style> 