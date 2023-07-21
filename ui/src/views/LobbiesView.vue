<script setup lang="ts">
import { getCurrentLobbies, type Lobby } from '@/client';
import type { AppAgentClient } from '@holochain/client';
import { computed, inject, ref, type ComputedRef, type Ref, onMounted } from 'vue';
import type { Record } from '@holochain/client';

const providedClient = inject<Ref<AppAgentClient>>('client');

const newLobbyName = ref('');

const lobbies = ref<Lobby[]>([])

onMounted(async () => {
  setTimeout(async () => {
    let client = providedClient?.value;
    if (!client) {
      console.log('no client yet');
      return;
    }

    try {
      const records = await getCurrentLobbies(client)
      lobbies.value = records;
    } catch (e) {
      console.error('Error getting current lobbies', e);
      return []
    }
  }, 300);
})

const createLobby = async () => {
  let client = providedClient?.value;
  if (!client) {
    return;
  }

  const lobby: Lobby = {
    name: newLobbyName.value,
    join_deadline: 5 * 60, // 5 minutes
    host: client.myPubKey,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'drone_swarm',
      zome_name: 'drone_swarm',
      fn_name: 'create_lobby',
      payload: lobby,
    });
    console.log('Created lobby', record);
  } catch (e: any) {
    // const errorSnackbar = this.$refs['create-error'] as Snackbar;
    // errorSnackbar.labelText = `Error creating the lobby: ${e.data.data}`;
    // errorSnackbar.show();
    console.error('Error creating the lobby', e);
  }
}

</script>

<template>
  <main>
    <h3>Lobbies</h3>

    <form>
      <input v-model="newLobbyName" type="text" placeholder="Lobby name" />
      <button type="button" @click="createLobby">Create</button>
    </form>

    <p>Found {{ lobbies.length }}</p>
    <ul v-for="lobby in lobbies" :key="lobby.name">
      <li>{{ lobby.name }}</li>
    </ul>
  </main>
</template>
