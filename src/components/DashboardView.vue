<script setup lang="ts">
import StatementsMonitor from "./DashboardViewFeatures/StatementsMonitor.vue";
import StatementsLog from "./DashboardViewFeatures/StatementsLog.vue";
import StatementsFilter from "./DashboardViewFeatures/StatementsFilter.vue";

</script>

<template>
  <div class="monitor-view-content">
    <div class="left-side">
      <div class="statements-monitor">
        <StatementsMonitor />
      </div>
      <div class="statements-log">
        <StatementsLog />
      </div>
    </div>
    <div class="right-side">
      <StatementsFilter />
    </div>
  </div>
</template>

<style scoped>
.monitor-view-content {
  display: flex;
  justify-content: space-between;
  box-sizing: border-box;
  width: 90vw;
  gap: 2vh;
}

.left-side {
  display: grid;
  flex: 2;
  row-gap: 2vh;
}

.right-side {
  flex: 1;
  height: 62vh;
}

.statements-monitor,
.statements-log {
  background-color: #314054;
  padding: 1rem;
  border-radius: 4px;
  height: 30vh;
}
</style>
