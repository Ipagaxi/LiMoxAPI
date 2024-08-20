<script setup lang="ts">
import { ref } from "vue";
import FilterItem from "./FilterItem.vue";

const filterItems = ref<Map<number, any>>(new Map()); // Array to hold the added components

function addFilter() {
  let currentFilterSize = filterItems.value.size;
  filterItems.value.set(currentFilterSize, FilterItem);
}

async function removeFilter(index: number) {
  filterItems.value.delete(index);
}

</script>

<template>
  <div class="statements-filter-container">
    <div class="label-container">
      Filter
    </div>
    <div class="filter-objects-container">
      <div
        v-for="[key, value] in filterItems"
        :key="key"
        class="filter-item-wrapper"
      >
        <component :is="value" @destroyFilter="removeFilter(key)"/>
      </div>
      <div class="filter-button-wrapper">
        <v-btn class="add-filter-button" @click="addFilter">
          <img
            src="./../../assets/plus.svg"
            class="plus-logo"
            alt="Add Filter"
          />
        </v-btn>
      </div>
    </div>
  </div>
</template>

<style scoped>

.statements-filter-container {
  padding-top: 3vh;
  background-color: #314054;
  border-radius: 4px;
  height: 100%;
  width: 100%;
  overflow-y: auto;
}

.label-container {
  font-size: calc(15px + 0.390625vw);;
  font-weight: bold;
  margin-bottom: 2vh;
  margin-left: 2%;
}

.filter-objects-container {
  margin: auto;
  width: 60%;
  display: grid;
  flex-wrap: wrap;
  gap: 1vw;
}

.filter-item-wrapper {
  border-radius: 4px;
}

.plus-logo {
  aspect-ratio: 1 / 1;
  width: 50%;
}

.filter-button-wrapper {
  flex: 1 1 auto;
  padding-top: 2vh;
  padding-bottom: 2vh;
}

.add-filter-button {
  background-color: #2b3a4e;
  height: 100%;
  width: 100%;
  padding-top: 2vh;
  padding-bottom: 2vh;
}
</style>
