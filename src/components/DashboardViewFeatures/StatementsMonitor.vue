<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { useToast } from "vue-toastification";

interface EventPayload {
  payload: {
    value: number;
  };
}

const toast = useToast();

const dataSize = 200;

const chartOptions = ref({
  chart: {
    id: "num-statement-chart",
    type: "line",
    animations: {
      enabled: true,
      easing: "linear",
      dynamicAnimation: {
        // Has to be the similar as the update frequency
        speed: 300,
      },
    },
    toolbar: {
      show: false
    }
  },
  title: {
    text: "Statement Pulse Monitor",
    align: "left",
    style: {
      fontSize: "15px",
      color:  "#e6e9fb"
    },
  },
  tooltip: {
    enabled: false
  },
  stroke: {
    curve: "smooth",
  },
  xaxis: {
    labels: {
      show: false,
    },
    range: 10,
  },
  yaxis: {
    seriesName: "Num. new statements",
    labels: {
      style: {
        colors: ['#e6e9fb']
      }
    }
  }
});
const chartSeries = ref([
  {
    name: "series-1",
    data: [3, 8, 2, 11],
  },
]);

function updateChart(newValue: number) {
  try {
    chartSeries.value[0].data.push(newValue);
    if (chartSeries.value[0].data.length > dataSize) {
      chartSeries.value[0].data = chartSeries.value[0].data.slice(-10);
    }
  } catch (error) {
    toast.error(`${error}`);
  }
}

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen(
    "prog-num-statements",
    (event: EventPayload) => {
      try {
        updateChart(event.payload.value);
      } catch (error) {
        toast.error(`${error}`);
      }
    },
  );
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
});
</script>

<template>
  <dev class="statements-monitor-container">
    <apexchart
      width="100%"
      height="100%"
      :options="chartOptions"
      :series="chartSeries"
    ></apexchart>
  </dev>
</template>

<style scoped>
.statements-monitor-container {
  display: flex;
  flex-grow: 1;
  height: 100%;
}

.statements-monitor-container > * {
  width: 100%;
}
</style>
