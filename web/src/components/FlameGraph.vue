<template>
  <div class="flame-container">
    <div ref="chartContainer" class="chart"></div>
    <div v-if="!hasData" class="placeholder">
      <div class="placeholder-content">
        <span class="placeholder-icon">📊</span>
        <h3>No Profile Data Available</h3>
        <p>Run a task to generate profile data</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted, computed } from 'vue'
import * as echarts from 'echarts'

interface ProfileNode {
  name: string;
  value: number;
  children?: ProfileNode[];
}

const props = defineProps<{
  profileData?: ProfileNode
  view?: 'flame' | 'sunburst'
}>()

const chartContainer = ref<HTMLElement | null>(null)
let chart: echarts.ECharts | null = null
const hasData = computed(() => Boolean(props.profileData?.value))
const currentView = computed(() => props.view || 'flame')

function createChart() {
  if (!chartContainer.value || chart) return
  chart = echarts.init(chartContainer.value)
  
  // Handle resize
  const resizeObserver = new ResizeObserver(() => {
    chart?.resize()
  })
  
  resizeObserver.observe(chartContainer.value)
  
  onUnmounted(() => {
    resizeObserver.disconnect()
    chart?.dispose()
  })
}

function processFlameData(node: ProfileNode, startX = 0, y = 0): any[] {
  const result = [{
    name: node.name.split('::').pop(),
    value: [startX, y, node.value, 1],  // [x, y, width, height]
    itemStyle: {
      color: y === 0 ? '#88ccee' : 
            y === 1 ? '#ee8866' :
            `hsl(${15 + y * 5}, 75%, ${65 - y * 3}%)`  // Red-orange gradient for deeper levels
    }
  }]

  if (node.children) {
    let currentX = startX
    node.children.forEach(child => {
      result.push(...processFlameData(child, currentX, y + 1))
      currentX += child.value
    })
  }

  return result
}

function updateChart(data: ProfileNode) {
  if (!chart) return

  const option = currentView.value === 'sunburst' ? {
    backgroundColor: '#fff',
    tooltip: {
      trigger: 'item',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderWidth: 1,
      borderColor: '#ddd',
      padding: [10, 15],
      formatter: ({ name, value }: any) => `
        <div style="padding: 8px">
          <div style="font-weight: bold">${name}</div>
          <div>Time: ${value}ms</div>
        </div>
      `
    },
    series: [{
      type: 'sunburst',
      data: [{
        name: data.name,
        value: data.value,
        children: data.children?.map(child => ({
          name: child.name,
          value: child.value,
          children: child.children
        }))
      }],
      radius: ['15%', '95%'],
      itemStyle: {
        borderWidth: 1,
        borderColor: '#fff'
      },
      label: {
        rotate: 'radial',
        fontSize: 14,
        color: '#333',
        minAngle: 15
      }
    }]
  } : {
    backgroundColor: '#fff',
    tooltip: {
      trigger: 'item',
      confine: true,
      backgroundColor: 'rgba(255, 255, 255, 0.98)',
      borderWidth: 1,
      borderColor: '#ddd',
      extraCssText: 'box-shadow: 0 2px 8px rgba(0,0,0,0.1);',
      padding: [12, 16],
      formatter: (params: any) => {
        const value = params.data.value[2]
        const percentage = ((value / data.value) * 100).toFixed(1)
        return `
          <div style="padding: 8px">
            <div style="font-weight: bold; margin-bottom: 4px">${params.name}</div>
            <div style="color: #666">Time: ${value}ms</div>
            <div style="color: #666">Percentage: ${percentage}%</div>
          </div>
        `
      }
    },
    grid: {
      top: '5%',
      right: '2%',
      bottom: '5%',
      left: '2%',
      containLabel: true
    },
    xAxis: {
      type: 'value',
      position: 'top',
      splitLine: { show: false },
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { show: false }
    },
    yAxis: {
      type: 'category',
      splitLine: { show: false },
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { show: false }
    },
    series: [{
      type: 'custom',
      renderItem: (params, api) => {
        const coords = api.coord([api.value(0), api.value(1)])
        const width = api.size([api.value(2), 0])[0]
        const height = 35
        const x = coords[0]
        const y = coords[1] - height / 2
        
        return {
          type: 'rect',
          shape: {
            x: x,
            y: y,
            width: width,
            height: height
          },
          style: {
            fill: api.style().fill,
            stroke: '#fff',
            lineWidth: 1
          }
        }
      },
      data: processFlameData(data),
      label: {
        show: true,
        position: 'inside',
        fontSize: 14,
        color: '#333',
        overflow: 'truncate',
        width: 100,
        formatter: (params: any) => {
          const value = params.data.value[2]
          const percentage = ((value / data.value) * 100).toFixed(1)
          if (percentage > 1.2) {
            return params.name
          }
          return ''
        }
      }
    }]
  }

  chart.setOption(option)
}

onMounted(() => {
  createChart()
  if (props.profileData) {
    updateChart(props.profileData)
  }
})

watch(() => props.profileData, (newData) => {
  if (newData) {
    updateChart(newData)
  }
}, { deep: true })

watch(() => props.view, () => {
  if (props.profileData) {
    updateChart(props.profileData)
  }
})
</script>

<style scoped>
.flame-container {
  width: 100%;
  height: 100%;
  position: relative;
  background: #fff;
  display: flex;
  flex-direction: column;
}

.chart {
  width: 100%;
  height: 100%;
}

.placeholder {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #fff;
  color: #666;
  pointer-events: none;
}

.placeholder-content {
  text-align: center;
}

.placeholder-icon {
  font-size: 48px;
  margin-bottom: 16px;
  display: block;
}

.placeholder-content h3 {
  margin: 0 0 8px 0;
  font-size: 20px;
  font-weight: 500;
  color: #2c3e50;
}

.placeholder-content p {
  margin: 0;
  color: #666;
  font-size: 15px;
}
</style> 