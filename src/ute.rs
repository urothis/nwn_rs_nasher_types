use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ute {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CreatureList")]
    pub creature_list: Option<NwValue<Vec<Creature>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Difficulty")]
    pub difficulty: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DifficultyIndex")]
    pub difficulty_index: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxCreatures")]
    pub max_creatures: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnEntered")]
    pub on_entered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExhausted")]
    pub on_exhausted: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExit")]
    pub on_exit: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayerOnly")]
    pub player_only: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RecCreatures")]
    pub rec_creatures: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Reset")]
    pub reset: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResetTime")]
    pub reset_time: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Respawns")]
    pub respawns: Option<NwValue<i16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SpawnOption")]
    pub spawn_option: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CR")]
    pub challenge_rating: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SingleSpawn")]
    pub single_spawn: Option<NwValue<u8>>,
}
