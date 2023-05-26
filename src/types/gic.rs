use serde::{Deserialize, Serialize};
use super::*;

/// Gic is a struct that represents area comments
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Gic {
    /// Creature List is a list of creatures
    ///
    #[serde(skip_serializing_if = "Option::is_none", rename = "Creature List")]
    pub creature_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Door List")]
    pub door_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Encounter List")]
    pub encounter_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "List")]
    pub list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Placeable List")]
    pub placeable_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SoundList")]
    pub sound_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "StoreList")]
    pub store_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TriggerList")]
    pub trigger_list: Option<NwValue<Vec<Comment>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "WaypointList")]
    pub waypoint_list: Option<NwValue<Vec<Comment>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayInToolset")]
    pub play_in_toolset: Option<NwValue<u8>>,
}
