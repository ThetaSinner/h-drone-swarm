<script setup lang="ts">
import { RouterView } from 'vue-router'
import { DSClient } from '@/client';
import { provide, ref } from 'vue';

let initialized = ref(false);
let initFailed = ref(false);

const dsClient = new DSClient();
provide('dsClient', dsClient);

dsClient.waitForConnected().then(() => {
  initialized.value = true;
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
