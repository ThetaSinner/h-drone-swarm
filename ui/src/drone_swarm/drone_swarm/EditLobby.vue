<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Lobby</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Host" :value="host" @input="host = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <vaadin-date-time-picker label="Join Deadline" :value="new Date(joinDeadline / 1000).toISOString()" @change="joinDeadline = new Date($event.target.value).valueOf() * 1000" required></vaadin-date-time-picker>
      </div>



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isLobbyValid"
        @click="updateLobby"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Lobby } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';

export default defineComponent({
  data(): {
    host: string;
    joinDeadline: number;
  } {
    const currentLobby = decode((this.currentRecord.entry as any).Present.entry) as Lobby;
    return { 
      host: currentLobby.host,
      joinDeadline: currentLobby.joinDeadline,
    }
  },
  props: {
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentLobby() {
      return decode((this.currentRecord.entry as any).Present.entry) as Lobby;
    },
    isLobbyValid() {
      return true && this.host !== '' && true;
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditLobby element`);
    }
  },
  methods: {
    async updateLobby() {

      const lobby: Lobby = { 
        host: this.host,
        join_deadline: this.joinDeadline,
        host: this.currentLobby.host,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'drone_swarm',
          zome_name: 'drone_swarm',
          fn_name: 'update_lobby',
          payload: {
            previous_lobby_hash: this.currentRecord.signed_action.hashed.hash,
            updated_lobby: lobby
          }
        });
        this.$emit('lobby-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the lobby: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['lobby-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
