<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
import { connectClient } from '@/client';
import { provide, ref } from 'vue';
import type { AppAgentClient } from '@holochain/client';

let initialized = ref(false);
let initFailed = ref(false);
let client = ref<AppAgentClient | null>(null);

provide('client', client);

connectClient().then((c) => {
  initialized.value = true;
  client.value = c;
}).catch(e => {
  console.error('Could not connect to Holochain', e);
  initFailed.value = true;
})

</script>

<template>
  <header>
    <div class="wrapper">
      <p v-if="initFailed">Failed to load, please refresh</p>
      <p v-if="initialized">App loaded, ok</p>
    </div>
  </header>

  <RouterView />
</template>
