import type { AppAgentClient, Record, RecordEntry } from "@holochain/client";
import type { Lobby } from ".";
import { decode } from '@msgpack/msgpack';

export const createLobby = async (client: AppAgentClient, lobby: Lobby): Promise<Record> => client.callZome({
    cap_secret: null,
    role_name: 'drone_swarm',
    zome_name: 'drone_swarm',
    fn_name: 'create_lobby',
    payload: lobby,
})

export const getCurrentLobbies = async (client: AppAgentClient): Promise<Lobby[]> => client.callZome({
    cap_secret: null,
    role_name: 'drone_swarm',
    zome_name: 'drone_swarm',
    fn_name: 'get_current_lobbies',
    payload: null,
}).then((records) => records.map((record: Record) => {
    return decode((record.entry as any).Present.entry)
}))
