<script setup lang="ts">
import StatementsMonitor from "./DashboardViewFeatures/StatementsMonitor.vue";
import StatementsLog from "./DashboardViewFeatures/StatementsLog.vue";
import StatementsFilter from "./DashboardViewFeatures/StatementsFilter.vue";
import StatementsInfos from "./DashboardViewFeatures/StatementsInfos.vue";

</script>

<template>
  <div class="monitor-view-content">
    <div class="left-side">
      <div class="top-container">
        <div class="statements-monitor">
          <StatementsMonitor />
        </div>
        <div class="statements-infos">
          <StatementsInfos/>
        </div>
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

.top-container {
  height: 30vh;
  display: flex;
  flex-direction: row;
  column-gap: 2vh;
}

.statements-monitor {
  flex: 3;
  border-radius: 4px;
  background-color: #314054;
  padding: 1rem;
}

.statements-infos {
  flex: 1;
  border-radius: 4px;
  background-color: #314054;
}

.statements-log {
  background-color: #314054;
  padding: 1rem;
  border-radius: 4px;
  height: 30vh;
}
</style>
