<script setup lang="ts">
import { DSClient, type Lobby } from '@/client';
import { State } from "@/stores";
import type { AppAgentClient } from '@holochain/client';
import { computed, inject, ref, type ComputedRef, type Ref, onMounted, toRaw } from 'vue';
import type { Record } from '@holochain/client';
import { useLobbiesStore } from '@/stores';

const providedClient = inject<DSClient>('dsClient');

const lobbiesStore = useLobbiesStore();

const newLobbyName = ref('');

const createLobby = async () => {
  await providedClient?.waitForConnected();

  const innerClient = providedClient?.innerClient;
  if (!innerClient) {
    return;
  }

  const lobby: Lobby = {
    name: newLobbyName.value,
    join_deadline: 5 * 60, // 5 minutes
    host: innerClient.myPubKey,
  };

  try {
    const record: Record = await innerClient.callZome({
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

    <div v-if="lobbiesStore.currentState !== State.Streaming">
      <p>Finding lobbies...</p>
    </div>
    <div v-else>
      <p>Found {{ lobbiesStore.lobbies.length }}</p>
      <ul v-for="lobby in lobbiesStore.lobbies" :key="lobby.name">
        <li>{{ lobby.name }}</li>
      </ul>
    </div>
  </main>
</template>
