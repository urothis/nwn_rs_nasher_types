use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fac {
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionList")]
    pub delay_entry: Option<NwValue<Vec<Faction>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RepList")]
    pub rep_list: Option<NwValue<Vec<Rep>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionGlobal")]
    pub faction_global: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionName")]
    pub faction_name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionParentID")]
    pub faction_parent_id: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rep {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionID1")]
    pub faction_id1: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionID2")]
    pub faction_id2: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FactionRep")]
    pub faction_rep: Option<NwValue<u8>>,
}
