use hdk::prelude::*;
use drone_swarm_integrity::*;
#[hdk_extern]
pub fn create_lobby(lobby: Lobby) -> ExternResult<Record> {
    let lobby_hash = create_entry(&EntryTypes::Lobby(lobby.clone()))?;
    let record = get(lobby_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Lobby"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_lobby(original_lobby_hash: ActionHash) -> ExternResult<Option<Record>> {
    get_latest_lobby(original_lobby_hash)
}
fn get_latest_lobby(lobby_hash: ActionHash) -> ExternResult<Option<Record>> {
    let details = get_details(lobby_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Lobby not found".into())))?;
    let record_details = match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(record_details),
    }?;
    if record_details.deletes.len() > 0 {
        return Ok(None);
    }
    match record_details.updates.last() {
        Some(update) => get_latest_lobby(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateLobbyInput {
    pub previous_lobby_hash: ActionHash,
    pub updated_lobby: Lobby,
}
#[hdk_extern]
pub fn update_lobby(input: UpdateLobbyInput) -> ExternResult<Record> {
    let updated_lobby_hash = update_entry(
        input.previous_lobby_hash,
        &input.updated_lobby,
    )?;
    let record = get(updated_lobby_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Lobby"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_lobby(original_lobby_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_lobby_hash)
}
