import { AppAgentWebsocket } from '@holochain/client';
import type { AppAgentClient } from '@holochain/client';

export * from './types';
export * from './api';

export const connectClient = async (): Promise<AppAgentClient> => AppAgentWebsocket.connect('', 'drone-swarm')
