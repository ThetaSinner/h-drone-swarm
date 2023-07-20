<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Lobby</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Host" :value="host" @input="host = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <vaadin-date-time-picker label="Join Deadline" :value="new Date(joinDeadline / 1000).toISOString()" @change="joinDeadline = new Date($event.target.value).valueOf() * 1000" required></vaadin-date-time-picker>
    </div>

  
    <mwc-button 
      raised
      label="Create Lobby"
      :disabled="!isLobbyValid"
      @click="createLobby"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Lobby } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
export default defineComponent({
  data(): {
    host: string;
    joinDeadline: number;
  } {
    return { 
      host: '',
      joinDeadline: Date.now(),
    }
  },
  props: {
    host: {
      type: null,
      required: true
    },
  },
  computed: {
    isLobbyValid() {
    return true && this.host !== '' && true;
    },
  },
  mounted() {
    if (this.host === undefined) {
      throw new Error(`The host input is required for the CreateLobby element`);
    }
  },
  methods: {
    async createLobby() {
      const lobby: Lobby = { 
        host: this.host!,

        join_deadline: this.joinDeadline!,

        host: this.host!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'drone_swarm',
          zome_name: 'drone_swarm',
          fn_name: 'create_lobby',
          payload: lobby,
        });
        this.$emit('lobby-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the lobby: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['lobby-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
