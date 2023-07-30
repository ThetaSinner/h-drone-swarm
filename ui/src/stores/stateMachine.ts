import { ref, type Ref } from "vue";

export const storageKey = (id: string) => `storeState#${id}`;

export enum State {
    RequiresInit,
    RequiresPatch,
    Streaming,
}

export class Transition {
    from: State;
    to: State;

    constructor(from: State, to: State) {
        this.from = from;
        this.to = to;
    }
}

const transitionKey = (t: Transition): string => {
    return `${t.from} -> ${t.to}`;
}

type TransitionFunction = (...args: unknown[]) => Promise<unknown>;

export class StoreStateMachine {
    private state: Ref<State> = ref(State.RequiresInit);
    private transitions: Map<string, TransitionFunction>;

    constructor(id: string, transitions: Map<Transition, Function>) {
        this.transitions = new Map();
        [...transitions.keys()].forEach(element => {
            this.transitions.set(transitionKey(element), transitions.get(element) as TransitionFunction);
        });

        if (localStorage.getItem(storageKey(id))) {
            // TODO not implemented
            // this.state.value = State.RequiresPatch;
            this.state.value = State.RequiresInit;
        } else {
            this.state.value = State.RequiresInit;
        }
    }

    get currentState(): Ref<State> {
        return this.state;
    }

    async transition(to: State, ...args: unknown[]): Promise<unknown> {
        const transition = new Transition(this.state.value, to);
        const key = transitionKey(transition);

        if (!this.transitions.has(key)) {
            throw new Error(`Invalid transition: ${key}, only have ${[...this.transitions.keys()]}`);
        }
        
        this.state.value = to;

        return this.transitions.get(key)?.call(null, ...args)
    }
}
