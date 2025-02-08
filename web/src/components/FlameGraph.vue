<template>
  <div ref="chart" style="width: 100%; height: 100%;"></div>
</template>

<script>
import { onMounted, ref } from 'vue';
import * as echarts from 'echarts';

const ColorTypes = {
    'root': '#8fd3e8',
    'fibonacci': '#d95850',  // Main function
    'main': '#eb8146',       // Entry point
    'std': '#ffb248',        // Standard library
    'core': '#f2d643',       // Core functions
    'alloc': '#ebdba4',      // Memory allocation
    'backtrace': '#fcce10',  // Stack traces
    'runtime': '#b5c334',    // Runtime functions
    'unknown': '#1bca93'     // Default color
};

const getColor = (name) => {
    for (const [key, color] of Object.entries(ColorTypes)) {
        if (name.toLowerCase().includes(key)) {
            return color;
        }
    }
    return ColorTypes.unknown;
};

export default {
  props: {
    profileId: String,
  },
  setup(props) {
    const chart = ref(null);
    let myChart = null;

    const filterJson = (json, id) => {
      if (id == null) {
        return json;
      }
      const recur = (item, id) => {
        if (item.id === id) {
          return item;
        }
        for (const child of item.children || []) {
          const temp = recur(child, id);
          if (temp) {
            item.children = [temp];
            item.value = temp.value; // change the parents' values
            return item;
          }
        }
      };
      return recur(json, id) || json;
    };

    const recursionJson = (jsonObj, id) => {
      const data = [];
      const filteredJson = filterJson(structuredClone(jsonObj), id);
      const rootVal = filteredJson.value;
      
      const recur = (item, start = 0, level = 0) => {
        const temp = {
          name: item.id,
          value: [
            level,
            start,
            start + item.value,
            item.name,
            (item.value / rootVal) * 100
          ],
          itemStyle: {
            color: getColor(item.name)
          }
        };
        data.push(temp);
        let prevStart = start;
        for (const child of item.children || []) {
          recur(child, prevStart, level + 1);
          prevStart = prevStart + child.value;
        }
      };
      recur(filteredJson);
      return data;
    };

    const heightOfJson = (json) => {
      const recur = (item, level = 0) => {
        if ((item.children || []).length === 0) {
          return level;
        }
        let maxLevel = level;
        for (const child of item.children) {
          const tempLevel = recur(child, level + 1);
          maxLevel = Math.max(maxLevel, tempLevel);
        }
        return maxLevel;
      };
      return recur(json);
    };

    const renderItem = (params, api) => {
      const level = api.value(0);
      const start = api.coord([api.value(1), level]);
      const end = api.coord([api.value(2), level]);
      const height = ((api.size && api.size([0, 1])) || [0, 20])[1];
      const width = end[0] - start[0];

      return {
        type: 'rect',
        transition: ['shape'],
        shape: {
          x: start[0],
          y: start[1] - height / 2,
          width,
          height: height - 2,
          r: 2
        },
        style: {
          fill: api.visual('color')
        },
        emphasis: {
          style: {
            stroke: '#000'
          }
        },
        textConfig: {
          position: 'insideLeft'
        },
        textContent: {
          style: {
            text: api.value(3),
            fontFamily: 'Verdana',
            fill: '#000',
            width: width - 4,
            overflow: 'truncate',
            ellipsis: '..',
            truncateMinChar: 1
          },
          emphasis: {
            style: {
              stroke: '#000',
              lineWidth: 0.5
            }
          }
        }
      };
    };

    onMounted(async () => {
      if (!chart.value) return;
      myChart = echarts.init(chart.value);
      myChart.showLoading();

      try {
        const response = await fetch(`http://[::1]:3000/api/profiles/${props.profileId}`);
        if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
        const profileData = await response.json();
        
        myChart.hideLoading();
        const levelOfOriginalJson = heightOfJson(profileData);

        const option = {
          backgroundColor: {
            type: 'linear',
            x: 0,
            y: 0,
            x2: 0,
            y2: 1,
            colorStops: [
              { offset: 0.05, color: '#eee' },
              { offset: 0.95, color: '#eeeeb0' }
            ]
          },
          tooltip: {
            formatter: (params) => {
              const samples = params.value[2] - params.value[1];
              return `${params.marker} ${params.value[3]}: (${samples} samples, ${params.value[4].toFixed(2)}%)`;
            }
          },
          title: [{
            text: 'Flame Graph',
            left: 'center',
            top: 10,
            textStyle: {
              fontFamily: 'Verdana',
              fontWeight: 'normal',
              fontSize: 20
            }
          }],
          toolbox: {
            feature: {
              restore: {}
            },
            right: 20,
            top: 10
          },
          xAxis: {
            show: false
          },
          yAxis: {
            show: false,
            max: levelOfOriginalJson
          },
          series: [{
            type: 'custom',
            renderItem,
            encode: {
              x: [0, 1, 2],
              y: 0
            },
            data: recursionJson(profileData)
          }]
        };

        myChart.setOption(option);
        myChart.on('click', (params) => {
          const data = recursionJson(profileData, params.data.name);
          const rootValue = data[0].value[2];
          myChart.setOption({
            xAxis: { max: rootValue },
            series: [{ data }]
          });
        });
      } catch (error) {
        console.error('Failed to fetch profile:', error);
        myChart.hideLoading();
      }
    });

    return {
      chart
    };
  }
};
</script>

<style scoped>
.chart {
  width: 100%;
  height: 100%;
  min-height: 600px;
}
</style> 