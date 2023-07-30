import type { AppAgentClient, AppSignal } from "@holochain/client";

export interface SignalPayload {
    type: string,
    action: object,
    app_entry: object,
    zome_name: string,
    cell_id: object,
}

type SignalCb = (payload: SignalPayload) => Promise<void>;

export class SignalBus {
    private client: AppAgentClient;
    private signalCbs: Map<string, SignalCb[]> = new Map();

    constructor(client: AppAgentClient) {
        this.client = client;

        this.client.on('signal', (signal) => {
            let raw_payload = signal.payload as SignalPayload;
            let payload: SignalPayload = {
                ...raw_payload,
                zome_name: signal.zome_name,
                cell_id: signal.cell_id,
            }

            console.log('received signal', payload);

            this.signalCbs.get(payload.type)?.forEach(cb => cb(payload));
        })
    }

    subscribeNewLobbies(cb: SignalCb) {
        console.log('subscribing for new lobbies');

        const key = 'NewLobby';
        if (this.signalCbs.has(key)) {
            this.signalCbs.get(key)?.push(cb);
        } else {
            this.signalCbs.set(key, [cb]);
        }
    }
}
