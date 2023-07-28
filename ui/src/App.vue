<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
import { SignalBus, connectClient } from '@/client';
import { provide, ref } from 'vue';
import type { AppAgentClient } from '@holochain/client';

let initialized = ref(false);
let initFailed = ref(false);
let client = ref<AppAgentClient | null>(null);
let signalBus = ref<SignalBus | null>(null);

provide('client', client);
provide('signalBus', signalBus);

connectClient().then((c) => {
  initialized.value = true;
  client.value = c;

  console.log('attach directly');
  c.on('signal', (s) => {
    console.log('direct signal', s);
  });

  console.log('attach directly: succeeded');

  signalBus.value = new SignalBus(c);
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
