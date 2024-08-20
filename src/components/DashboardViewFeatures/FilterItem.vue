<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineEmits, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useToast } from "vue-toastification";

import useSound from "vue-use-sound";
import sound1 from "../../assets/Sounds/sound1.wav";
import sound2 from "../../assets/Sounds/sound2.wav";
import sound3 from "../../assets/Sounds/sound3.wav";
import sound4 from "../../assets/Sounds/sound4.wav";

interface FilterRule {
  path: string;
  value: string;
  option: string;
}

interface EventPayload {
  payload: {
    value: number;
  };
}

const toast = useToast();

const emit = defineEmits(['destroyFilter']);

const filteredStatementsCounter = ref<number>(0);
const id = 'id' + (new Date()).getTime();
const filterRules = ref<FilterRule[]>([]); // Array to hold the added components

const filterLabel = ref<string>("filter 1");

const configDialogIsActive = ref(false)

const selectedSound = ref<string>("Sound 1");
// There were problems to load the sound dynamically when they are needed so every sound gets loaded in the beginning
let [playSound1] = useSound(sound1);
let [playSound2] = useSound(sound2);
let [playSound3] = useSound(sound3);
let [playSound4] = useSound(sound4);


function playSelectedSound() {
  switch (selectedSound.value) {
    case 'Sound 1': {
      playSound1();
      break;
    }
    case 'Sound 2': {
      playSound2();
      break;
    }
    case 'Sound 3': {
      playSound3();
      break;
    }
    case 'Sound 4': {
      playSound4();
      break;
    }
  }
}

watch(selectedSound, () => {
  playSelectedSound();
})

async function addFilterValue() {
  filterRules.value.push({ path: "", value: "", option: "In" });
  await invoke("update_filters", {id: id, filterRules: filterRules.value});
}

function deleteRule(index: number) {
  filterRules.value.splice(index, 1);
}


function deleteFilter() {
  emit('destroyFilter');
}

async function updateFilterAndCloseDialog() {
  try {
    await invoke("update_filters", {id: id, filterRules: filterRules.value});
    configDialogIsActive.value = false
  } catch (error) {
    toast.error(`${error}`);
  }
}

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen(
    id + "-filtered-statements-counter",
    (event: EventPayload) => {
      try {
        let result: number = event.payload.value;
        if (result > filteredStatementsCounter.value) {
          playSelectedSound();
        }
        filteredStatementsCounter.value = result;
      } catch (error) {
        toast.error(`${error}`);
      }
    },
  );
  addFilterValue();
});

onUnmounted(() => {
  try {
    invoke("deregister_filter", { filterId: id });
  } catch (error) {
    toast.error(`${error}`);
  }
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
});

</script>

<template>
  <div class="filter-item-wrapper_">
    <v-container class="grid-container">
      <v-row no-gutters>
        <v-col cols="9">
          <div class="filter-label-container">
            {{ filterLabel }}
          </div>
          <div class="statements-counter-container">
            {{ filteredStatementsCounter }}
          </div>
        </v-col>
        <v-col>
          <div class="item-bar">
            <v-icon name="bi-x-circle-fill" class="remove-filter-btn" color="#e6e9fb" @click="deleteFilter()"></v-icon>
            <v-dialog v-model="configDialogIsActive">
              <template v-slot:activator="{ props: activatorProps }">
                  <v-icon name="bi-gear-fill" class="config-btn" v-bind="activatorProps" color="#e6e9fb"></v-icon>
              </template>

              <template v-slot:default>
                <v-card title="Filter Configs" color="#2b3a4e">
                  <v-container>
                    <v-row>
                      <v-col>
                        <v-text-field 
                          label="label"
                          class="set-label-field"
                          v-model="filterLabel"
                        ></v-text-field>
                      </v-col>
                      <v-col>
                        <v-select
                          label="Sound"
                          theme="dark"
                          v-model="selectedSound"
                          :items="['Disabled', 'Sound 1', 'Sound 2', 'Sound 3', 'Sound 4']"
                          variant="solo-filled"
                        ></v-select>
                      </v-col>
                    </v-row>
                    <v-row v-for="(_,index) in filterRules" :key="index">
                      <v-col cols="1">
                          <v-icon name="bi-x-circle-fill" @click="deleteRule(index)"></v-icon>
                      </v-col>
                      <v-col>
                          <v-select
                            label="Filter Option"
                            theme="dark"
                            v-model="filterRules[index].option"
                            :items="['In', 'Out', 'Disabled']"
                            variant="solo-filled"
                          ></v-select>
                      </v-col>
                      <v-col>
                          <v-text-field
                            label="Value Path"
                            bg-color="#1d232b"
                            class="value-path-field"
                            v-model="filterRules[index].path"
                            :disabled="filterRules[index].option === 'Disabled'"
                          ></v-text-field>
                      </v-col>
                      <v-col>
                          <v-text-field
                            label="Value"
                            bg-color="#1d232b"
                            class="value-field"
                            v-model="filterRules[index].value"
                            :disabled="filterRules[index].option === 'Disabled'"
                          ></v-text-field>
                      </v-col>
                    </v-row>
                    <v-row>
                      <v-col>
                          <v-btn class="add-filter-rule-btn" text="Add" @click="addFilterValue"></v-btn>
                      </v-col>
                    </v-row>
                  </v-container>
                  <v-card-actions>
                      <v-btn
                        text="Save & Close"
                        class="save-close-btn"
                        @click="updateFilterAndCloseDialog"
                      ></v-btn>
                    </v-card-actions>
                </v-card>
              </template>
            </v-dialog>
          </div>    
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<style scoped>
.filter-item-wrapper_ {
  background-color: #2b3a4e;
  border-radius: 4px;
  display: grid;
  height: 100%;
  width: 100%;
}

.grid-container {
  padding-top: 0;
}

.filter-label-container {
  color: #e6e9fb;
  font-size: calc(10px + 0.390625vw);
}

.statements-counter-container {
  margin-top: 1vh;
  color: #e6e9fb;
  font-size: calc(30px + 0.390625vw);
}

.item-bar {
  padding-top: 1vh;
  display: grid;
}

.remove-filter-btn {
  margin: auto;
}

.config-btn {
  margin: auto;
}

.set-label-field {
  color: #e6e9fb;
}

.value-path-field {
  color: #e6e9fb;
}

.value-field {
  color: #e6e9fb;
}

.add-filter-rule-btn {
  background-color: #1d232b;
  color: #e6e9fb;
  padding-left: 5vw;
  padding-right: 5vw;
}

.save-close-btn {
  background-color: #1d232b;
  color: #e6e9fb;
}
</style>