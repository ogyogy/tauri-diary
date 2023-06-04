<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const name = ref("");
const items = ref([]);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  items.value.push(await invoke("greet", { name: name.value }));
}

async function reload() {
  items.value.splice(0);
}
</script>

<template>
  <v-form @submit.prevent="greet()">
    <v-text-field v-model="name" label="Message" type="input" variant="underlined" prepend-icon="mdi-reload"
      append-icon="mdi-send" clearable @click:prepend="reload()" @click:append="greet()">
    </v-text-field>
  </v-form>
  <v-virtual-scroll :height="400" :items="items">
    <template v-slot:default="{ item }">
      <v-card class="mx-auto">
        <v-card-subtitle>
          {{ item[0] }}
        </v-card-subtitle>
        <v-card-text>
          {{ item[1] }}
        </v-card-text>
      </v-card>
    </template>
  </v-virtual-scroll>
</template>
