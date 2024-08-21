<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";

interface NumPayload {
  payload: {
    value: number;
  };
}

interface DateTimePayload {
  payload: {
    value: string;
  };
}

const numStatements = ref<number>(0);
const latestStatementDateTime = ref<string>("");
const numActors = ref<number>(0);

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen(
    "prog-num-statements",
    (event: NumPayload) => {
      numStatements.value += event.payload.value;
    },
  );
  unlisten = await listen(
    "datetime-event",
    (event: DateTimePayload) => {
      latestStatementDateTime.value = event.payload.value;
    },
  );
  unlisten = await listen(
    "num-actors-event",
    (event: NumPayload) => {
      numActors.value = event.payload.value;
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
  <div class="info-container">
    <div class="info-item">
        <div class="info-labels">
            Latest Statement
        </div>
        <div class="latest-statement-datetime">
            {{ latestStatementDateTime }}
        </div>
    </div>
    <div class="info-item">
        <div class="info-labels">
            Statements
        </div>
        <div class="num-value">
            {{ numStatements }}
        </div>
    </div>
    <div class="info-item">
        <div class="info-labels">
            Actors
        </div>
        <div class="num-value">
            {{ numActors }}
        </div>
    </div>
  </div>
</template>

<style scoped>

.info-container {
  display: flex;
  flex-direction: column;
  background-color: #314054;
  border-radius: 4px;
  height: 100%;
  width: 100%;
}


.info-item {
    flex: 1;
}

.info-labels {
    font-size: calc(8px + 0.390625vw);
}

.num-values {
    font-size: calc(20px + 0.390625vw);
}
</style>
