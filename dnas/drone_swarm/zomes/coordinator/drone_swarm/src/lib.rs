pub mod lobby;
use drone_swarm_integrity::*;
use hdk::prelude::*;

mod signal;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

// After an action is committed, we emit a signal to the UI elements to reactively update them
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal::signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
