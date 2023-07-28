import type { AppAgentClient, AppSignal } from "@holochain/client";

export class SignalBus {
    private client: AppAgentClient;

    constructor(client: AppAgentClient) {
        console.log('bus this consructor', this);
        this.client = client;

        // this.subscribeNewLobbies = this.subscribeNewLobbies.bind(this);
    }

    subscribeNewLobbies(cb: (signal: AppSignal) => void) {
        console.log('bus this is', this);
        this.client.on('signal', cb);
    }
}
