<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditLobby
        :current-record="record!"
        @lobby-updated="editing = false; fetchLobby();"
        @edit-canceled="editing = false"
      ></EditLobby>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteLobby()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Host: </strong></span>
 	<span style="white-space: pre-line">{{  lobby?.host }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Join Deadline: </strong></span>
 	<span style="white-space: pre-line">{{  new Date(lobby?.join_deadline / 1000).toLocaleString() }} </span>
      </div>

    </div>
    
    <span v-else>The requested lobby was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Lobby } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditLobby from './EditLobby.vue';

export default defineComponent({
  components: {
    EditLobby
  },
  props: {
    lobbyHash: {
      type: Object,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
    }
  },
  computed: {
    lobby() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Lobby;
    }
  },
  async mounted() {
    if (this.lobbyHash === undefined) {
      throw new Error(`The lobbyHash input is required for the LobbyDetail element`);
    }

    await this.fetchLobby();
  },
  methods: {
    async fetchLobby() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'drone_swarm',
        zome_name: 'drone_swarm',
        fn_name: 'get_lobby',
        payload: this.lobbyHash,
      });

      this.loading = false;
    },
    async deleteLobby() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'drone_swarm',
          zome_name: 'drone_swarm',
          fn_name: 'delete_lobby',
          payload: this.lobbyHash,
        });
        this.$emit('lobby-deleted', this.lobbyHash);
        this.fetchLobby();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the lobby: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['lobby-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
