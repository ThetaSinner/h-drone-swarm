import { AppAgentWebsocket } from '@holochain/client';
import type { AppAgentClient, Record } from "@holochain/client";
import { type Lobby } from ".";
import { decode } from '@msgpack/msgpack';
import { SignalBus } from './signal';

export * from './types';
export * from './signal';

export const connectClient = async (): Promise<AppAgentClient> => AppAgentWebsocket.connect('', 'drone-swarm')

export class DSClient {
    private client: AppAgentClient | null = null;
    private sb: SignalBus | null = null;

    constructor() {
        connectClient().then((client) => {
            this.client = client;
            this.sb = new SignalBus(client);
        });
    }

    get signalBus() {
        return this.sb
    }

    get innerClient() {
        return this.client;
    }

    async waitForConnected(): Promise<null> {
        console.log('waiting for client')

        if (this.client) {
            return Promise.resolve(null);
        }

        return new Promise((resolve, reject) => {
            const interval = setInterval(() => {
                if (this.client) {
                    clearInterval(interval);
                    resolve(null);
                    return;
                }
            }, 10);
    
            setTimeout(() => {
                clearInterval(interval);
                reject(new Error('Failed to connect to holochain'));
            }, 5000);
        });
    }

    async createLobby(lobby: Lobby): Promise<Record> {
        return this.client?.callZome({
            cap_secret: null,
            role_name: 'drone_swarm',
            zome_name: 'drone_swarm',
            fn_name: 'create_lobby',
            payload: lobby,
        })
    }

    async getCurrentLobbies(): Promise<Lobby[]> {
        const records = await this.client?.callZome({
            cap_secret: null,
            role_name: 'drone_swarm',
            zome_name: 'drone_swarm',
            fn_name: 'get_current_lobbies',
            payload: null,
        });
        
        return Promise.resolve(records.map((record: Record) => {
            return decode((record.entry as any).Present.entry)
        }))
    }

}
