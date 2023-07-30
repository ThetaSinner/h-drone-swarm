import { defineStore } from "pinia"
import { inject, ref } from "vue"
import { StoreStateMachine, State, Transition } from "./stateMachine"
import { DSClient, type Lobby, type SignalPayload } from "@/client";
import type { Sign } from "crypto";

const STORE_KEY = 'lobbies';

export const useLobbiesStore = defineStore(STORE_KEY, () => {
    const lobbies = ref<Lobby[]>([]);
    const client = inject<DSClient>('dsClient');

    let transitions = new Map();
    transitions.set(new Transition(State.RequiresInit, State.Streaming), async () => {
        await client?.waitForConnected()

        console.log('client connected');

        const currentLobbies = await client?.getCurrentLobbies();

        console.log('got lobbies');

        if (currentLobbies) {
            localStorage.setItem(STORE_KEY, JSON.stringify(currentLobbies));
            lobbies.value = currentLobbies;
        } else {
            console.error('Failed to get current lobbies');
        }
    })

    transitions.set(new Transition(State.Streaming, State.Streaming), (signal: SignalPayload) => {
        // TODO handle other signal types here for updates or deletes etc

        let newLobby = signal.app_entry as Lobby;
        let updatedLobbies = [...lobbies.value, newLobby];

        localStorage.setItem(STORE_KEY, JSON.stringify(updatedLobbies));
        lobbies.value = updatedLobbies;
    })

    const stateMachine = new StoreStateMachine('lobbies', transitions);

    client?.waitForConnected().then(() => {
        client?.signalBus?.subscribeNewLobbies(async (signal) => {
            console.log('updating with signal', signal);

            stateMachine.transition(State.Streaming, signal);
        });
    });

    // Try to launch the state machine
    stateMachine.transition(State.Streaming);

    return { lobbies, currentState: stateMachine.currentState }
})
