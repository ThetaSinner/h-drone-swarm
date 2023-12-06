use drone_swarm_integrity::*;
use hdk::prelude::{*, holo_hash::AnyLinkableHashPrimitive, tracing::warn};

#[hdk_extern]
pub fn create_lobby(lobby: Lobby) -> ExternResult<Record> {
    info!("Creating lobby: {:?}", lobby);

    let lobby_advertise_windows = get_lobby_advertise_windows()?
        .into_iter()
        .map(|w| hash_entry(&LobbyWindow::from(w)))
        .collect::<ExternResult<Vec<EntryHash>>>()?;

    let lobby_hash = create_entry(&EntryTypes::Lobby(lobby))?;
    let record = get(lobby_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created Lobby"))
    ))?;

    for lobby_window_hash in lobby_advertise_windows {
        create_link(
            lobby_window_hash,
            lobby_hash.clone(),
            LobbyLinkTypes::LobbyWindowToLobby,
            (),
        )?;
    }

    info!("Created lobby: {:?}", record);

    Ok(record)
}

#[hdk_extern]
pub fn get_current_lobbies(_: ()) -> ExternResult<Vec<Record>> {
    info!("Getting current lobbies");

    let lobby_advertise_windows = get_lobby_advertise_windows()?
        .into_iter()
        .map(|w| hash_entry(LobbyWindow::from(w)))
        .collect::<ExternResult<Vec<EntryHash>>>()?;

    info!("Found lobby advertise windows: {:?}", lobby_advertise_windows);

    let links = lobby_advertise_windows
        .into_iter()
        .map(|base| get_links(GetLinksInputBuilder::try_new(base, LobbyLinkTypes::LobbyWindowToLobby)?.build()))
        .collect::<ExternResult<Vec<Vec<Link>>>>()?;

    info!("Found lobby links: {:?}", links);

    Ok(links.into_iter().flatten().map(|link| {
        info!("Processing link {:?}", link);

        // TODO `into_any_dht_hash` not available until a later version of the hdk
        match link.target.into_primitive() {
            AnyLinkableHashPrimitive::Action(action_hash) => {
                info!("Getting lobby action: {:?}", action_hash);
                get(action_hash, GetOptions::default())
            }
            _ => {
                warn!("Lobby link target is not an ActionHash");
                Ok(None)
            }
        }
    }).collect::<ExternResult<Vec<Option<Record>>>>()?.into_iter().filter_map(|v| v).collect())
}

#[hdk_extern]
pub fn get_lobby(original_lobby_hash: ActionHash) -> ExternResult<Option<Record>> {
    get_latest_lobby(original_lobby_hash)
}

fn get_latest_lobby(lobby_hash: ActionHash) -> ExternResult<Option<Record>> {
    let details = get_details(lobby_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Lobby not found".into())))?;
    let record_details = match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
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
    let updated_lobby_hash = update_entry(input.previous_lobby_hash, &input.updated_lobby)?;
    let record = get(updated_lobby_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly updated Lobby"))
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_lobby(original_lobby_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_lobby_hash)
}

const WINDOW_SIZE_SECONDS: i64 = 60 * 60; // 1 hour
const WINDOW_OVERLAP_SECONDS: i64 = 10 * 60; // 10 minutes

fn get_lobby_advertise_windows() -> ExternResult<Vec<i64>> {
    let timestamp = sys_time()?;
    let current_seconds = timestamp.as_seconds_and_nanos().0;

    let current_offset = current_seconds % WINDOW_SIZE_SECONDS;

    let window_mid = current_seconds - current_offset + WINDOW_SIZE_SECONDS / 2;

    if current_offset < WINDOW_OVERLAP_SECONDS {
        let previous_window_mid = window_mid - WINDOW_SIZE_SECONDS;
        Ok(vec![previous_window_mid, window_mid])
    } else if current_offset > WINDOW_SIZE_SECONDS - WINDOW_OVERLAP_SECONDS {
        let next_window_mid = window_mid + WINDOW_SIZE_SECONDS;
        Ok(vec![window_mid, next_window_mid])
    } else {
        Ok(vec![window_mid])
    }
}
