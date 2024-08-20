<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineEmits } from "vue";
import { listen } from "@tauri-apps/api/event";
import { emit } from '@tauri-apps/api/event';
import { useToast } from "vue-toastification";

interface ConnectionPayload {
  ip_address: string;
  port: string;
  username: string;
  password: string;
}

interface ConnectionResponsePayload {
  payload: {
    successfull: boolean;
    response: string;
  };
}

const vueEmit = defineEmits(['connectionEstablished']);

const ipAddress = ref<string>("127.0.0.1");
const port = ref<string>("8080");
const username = ref<string>(
  "db3e9b32e6c2ceeb06c69a98ad3315c0137999172a4c3e56339bb61c94340872",
);
const password = ref<string>(
  "30cfcff6247a86c8023968a30956c8cea6a43e416faeb96c30fb7c9d4a10e46e",
);

const toast = useToast();

const inConnectionProgress = ref<boolean>(false);

async function connectToLRS() {
  inConnectionProgress.value = true;
  try {
    const connectionPayload: ConnectionPayload = {
      ip_address: ipAddress.value,
      port: port.value,
      username: username.value,
      password: password.value
    }
    emit('connect-lrs', connectionPayload);
  } catch (error) {
    toast.error(`${error}`);
  }
}

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen(
    "connection-response",
    (event: ConnectionResponsePayload) => {
      inConnectionProgress.value = false;
      try {
        let succ_connected: boolean = event.payload.successfull;
        let response: string = event.payload.response;
        if (succ_connected) {
          toast.success("Connected to LRS successfully!");
          vueEmit('connectionEstablished');
        } else {
          toast.error("Failed to Connect: " + response);
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
  <v-sheet class="config-sheet mx-auto" width="300">
    <v-form class="form" fast-fail @submit.prevent="connectToLRS">
      <v-text-field
        v-model="ipAddress"
        label="IP Address"
        bg-color="#2b3a4e"
        class="form-field"
      ></v-text-field>
      <v-text-field
        v-model="port"
        label="Port"
        bg-color="#2b3a4e"
        class="form-field"
      ></v-text-field>
      <v-text-field
        v-model="username"
        label="Username"
        bg-color="#2b3a4e"
        class="form-field"
      ></v-text-field>
      <v-text-field
        v-model="password"
        label="Password"
        bg-color="#2b3a4e"
        class="form-field"
      ></v-text-field>

      <v-btn class="mt-2 connect-btn" :loading="inConnectionProgress" type="submit" block>Connect</v-btn>
    </v-form>
  </v-sheet>
</template>

<style scoped>
.config-sheet {
  background-color: #314054;
  border-radius: 12px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.v-text-field {
  margin-bottom: 10px;
}

.form-field {
  color: #e6e9fb;
}

.connect-btn {
  background-color: #28a745;
  color: #fff;
  padding: 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.connect-btn:hover {
  background-color: #218838;
}
</style>