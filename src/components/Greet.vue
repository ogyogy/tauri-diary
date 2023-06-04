<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Message {
  id: number;
  text: string;
  created_at: string;
}

const text = ref("");
const items = ref<Message[]>([]);

function convert(value: { id: any; text: any; created_at: any; }) {
  // 生成時刻をUNIX時間からローカルタイムゾーンの文字列に変換
  // Dateの引数に与えるUNIX時間はミリ秒単位
  let created_at: Date = new Date(value.created_at * 1000);
  return {
    id: value.id,
    text: value.text,
    created_at: created_at.toString(),
  }
}

onMounted(async () => {
  let data: { id: number, text: string, created_at: number }[] = await invoke("get_messages_all");
  // 生成時刻を基準に降順ソート
  data.sort((a, b) => b.created_at - a.created_at)
  // 新しく生成したメッセージを上に表示する
  // 生成時刻をUNIX時間からローカルタイムゾーンの文字列に変換
  items.value = data.map(value => {
    return convert(value);
  })
})

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let msg: Message = convert(await invoke("greet", { text: text.value }));
  // 新しく生成したメッセージを上に表示する
  // 配列の先頭にメッセージを追加
  items.value.unshift(msg);
  // テキストフィールドを空にする
  text.value = "";
}
</script>

<template>
  <v-form @submit.prevent="greet()">
    <v-text-field v-model="text" placeholder="Write a message" type="input" variant="underlined" append-icon="mdi-send"
      clearable @click:append="greet()">
    </v-text-field>
  </v-form>
  <v-virtual-scroll :height="400" :items="items">
    <template v-slot:default="{ item }">
      <v-card class="mx-auto">
        <v-card-subtitle>
          {{ item.created_at }}
        </v-card-subtitle>
        <v-card-text>
          {{ item.text }}
        </v-card-text>
      </v-card>
    </template>
  </v-virtual-scroll>
</template>
