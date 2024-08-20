<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import ConfigView from "./components/ConfigView.vue";
import DashboardView from "./components/DashboardView.vue";
import { useToast } from "vue-toastification";

interface ConnectionResponsePayload {
  payload: {
    successfull: boolean;
    response: string;
  };
}

const toast = useToast();

type ViewName = "DashboardView" | "ConfigView";

const views: Record<ViewName, any> = {
  DashboardView,
  ConfigView,
};

const buttonName = ref("Dashboard");

const connectColor = ref<string>("red");
const connectionLabel = ref<string>("Not connected");

const currentView = ref<ViewName>("ConfigView");

const swapView = () => {
  if (currentView.value === "DashboardView") {
    currentView.value = "ConfigView";
    buttonName.value = "Dashboard";
  } else {
    currentView.value = "DashboardView";
    buttonName.value = "Back";
  }
};

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen(
    "connection-response",
    (event: ConnectionResponsePayload) => {
      try {
        let succ_connected: boolean = event.payload.successfull;
        let response: string = event.payload.response;
        if (succ_connected) {
          connectColor.value = "green";
          connectionLabel.value = "Connected"
        } else {
          connectColor.value = "red";
          connectionLabel.value = "Not Connected";
        }
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
  <div class="container">
    <h1 class="app-heading">LiMoXApi</h1>
    <div class="connection-display">
      <v-card :color="connectColor">
        <v-card-title>{{ connectionLabel }}</v-card-title>
      </v-card>
    </div>
    <div class="view-content">
      <button class="swap-button" @click="swapView">{{ buttonName }}</button>
      <!-- Use the views mapping to dynamically select the component -->
      <component :is="views[currentView]" @connectionEstablished="swapView"></component>
    </div>
  </div>
</template>

<style scoped>


.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  background-color: #2b3a4e;
  font-family: "Roboto", sans-serif;
}

.view-content {
  box-sizing: border-box;
  position: relative;
  flex: 1;
  padding: 1rem;
}

.app-heading {
  color: #e6e9fb;
  font-size: 2.5rem;
  margin: 0;

  top: 0;
  width: 100%;
  background-color: #2b3a4e;
  padding: 1rem;
  text-align: center;
}

.connection-display {
  position: absolute;
  top: 20px;
  right: 40px;
}

.swap-button {
  position: relative;
  background-color: #314054;
  color: #fff;
  border: none;
  border-radius: 4px;
  padding: 10px 30px;
  margin-bottom: 10px;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.3s;
  width: 150px;
}

.swap-button:hover {
  background-color: #131920;
}
</style>
