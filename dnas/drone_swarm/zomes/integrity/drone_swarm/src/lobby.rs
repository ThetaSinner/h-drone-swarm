use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct LobbyWindow {
    pub mid_seconds: i64,
}

impl From<i64> for LobbyWindow {
    fn from(mid_seconds: i64) -> Self {
        Self { mid_seconds }
    }
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Lobby {
    pub name: String,
    pub join_deadline: Timestamp,
    pub host: AgentPubKey,
}

pub fn validate_create_lobby(
    _action: EntryCreationAction,
    _lobby: Lobby,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_lobby(
    _action: Update,
    _lobby: Lobby,
    _original_action: EntryCreationAction,
    _original_lobby: Lobby,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_lobby(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_lobby: Lobby,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
