import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleLobby(cell: CallableCell, partialLobby = {}) {
    return {
        ...{
	  host: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  join_deadline: 1674053334548000,
	  host: (await fakeAgentPubKey()),
        },
        ...partialLobby
    };
}

export async function createLobby(cell: CallableCell, lobby = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "drone_swarm",
      fn_name: "create_lobby",
      payload: lobby || await sampleLobby(cell),
    });
}

