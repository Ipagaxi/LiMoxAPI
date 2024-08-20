<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, emit } from "@tauri-apps/api/event";
import { useToast } from "vue-toastification";

interface EventPayload {
  payload: {
    densed_statements: string[];
    full_statements: string[];
  };
}

interface VirtualScrollComponent {
  scrollToIndex: (index: number) => void;
}

const toast = useToast();

const condensedStatements = ref<string[]>([
  ""
]);

const fullStatements = ref<string[]>([]);

const currentLogIndex = ref<number>(0);

const fullStatementShown = ref<boolean>(false);
const currentPos = ref(0);
const numLogs = ref(0);

const logSize = 2000;
let virtualScroll = ref<VirtualScrollComponent | null>(null);
const isAtBottom = () => {
  if (!virtualScroll.value) return false;
  const element = virtualScroll.value.$el;
  let pos = element.scrollTop + (1.5*element.clientHeight);
  currentPos.value = pos;
  numLogs.value = element.scrollHeight;
  let isBottom = pos >= element.scrollHeight;
  return isBottom;
};

async function initWithExistingData() {
  try {
    const result: string[] = await invoke("send_existing_data_to_log");
    condensedStatements.value = result;
  } catch (error) {
    toast.error(`${error}`);
  }
}

function showFullStatement(index: number) {
  if (index < condensedStatements.value.length -1) {
    fullStatementShown.value = true;
    currentLogIndex.value = index;
  }
}

function scrollToEnd() {
  nextTick(() => {
    let lastIndex = condensedStatements.value.length - 1;
    emit("log", "Scroll to: " + lastIndex);
    if (virtualScroll.value) {
      try {
        virtualScroll.value.scrollToIndex(lastIndex);
        // Additional check to ensure that the scroll reached the end
        if (!isAtBottom()) {
          virtualScroll.value.scrollToIndex(lastIndex); // Retry if necessary
        }
      } catch (error) {
        emit("log", `${error}`);
      }
    }
  });
}

let unlisten: (() => void) | null = null;

const log = ref("else");

onMounted(async () => {
  //initWithExistingData();
  unlisten = await listen(
    "log-statements-event",
    async (event: EventPayload) => {
      try {
        let newDensedStatements: string[] = event.payload.densed_statements;
        condensedStatements.value.pop();
        condensedStatements.value.push(...newDensedStatements);
        condensedStatements.value.push("---");
        emit("log", "Log size: " + condensedStatements.value.length)
        if (isAtBottom() && !(virtualScroll.value === null)) {
          await nextTick();
          scrollToEnd();
        } else {
          emit('log', "Not at bottom");
        }
        let newFullStatements: string[] = event.payload.full_statements;
        
        fullStatements.value.push(...newFullStatements);
        /*if (condensedStatements.value.length > logSize) {
          condensedStatements.value = condensedStatements.value.slice(-logSize);
          fullStatements.value = fullStatements.value.slice(-logSize);
        }*/
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
  <dev class="statements-log-container">
    <dev class="label-container" @click="scrollToEnd">
      Log
      Current Pos: {{ currentPos }}
      Num Logs: {{ numLogs }}
    </dev>
    <v-virtual-scroll
      class="virtual-scroll"
      :items="condensedStatements"
      ref="virtualScroll"
    >
      <template v-slot:default="{ item, index }">
        <div @click="showFullStatement(index)">
          {{ item }}
        </div>
      </template>
    </v-virtual-scroll>
    <v-dialog v-model="fullStatementShown">
      <template v-slot:default="{ isActive }">
        <v-card title="Full Statement" color="#2b3a4e">
          <v-card-text>
            <div class="scrollable-container">
              <pre v-html="fullStatements[currentLogIndex]"></pre>
            </div>
          </v-card-text>
        </v-card>
      </template>
    </v-dialog>
  </dev>
</template>

<style scoped>
.statements-log-container {
  color: #e6e9fb;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.scrollable-container {
  overflow: auto;
}

.label-container {
  text-align: left;
  font-size: 15px;
  font-weight: bold;
  margin-bottom: 2vh;
}

.virtual-scroll {
  flex: 1;
  overflow-y: auto;
  font-size: calc(10px + 0.390625vw);
  text-align: left;
}
</style>
