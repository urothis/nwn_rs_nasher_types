//! # Faction
//! Structs for the `fac` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents a faction in the `fac` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Fac {
  /// Field representing the list of factions.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionList")]
  pub faction_list: Option<NwValue<Vec<Faction>>>,

  /// Field representing the list of reputation entries.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "RepList")]
  pub rep_list: Option<NwValue<Vec<Rep>>>,
}

/// Represents a faction entry in the `fac` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
  /// Field representing the struct ID.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// Field representing the faction global value.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionGlobal")]
  pub faction_global: Option<NwValue<u16>>,

  /// Field representing the faction name.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionName")]
  pub faction_name: Option<NwValue<String>>,

  /// Field representing the faction's parent ID.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionParentID")]
  pub faction_parent_id: Option<NwValue<u32>>,
}

/// Represents a reputation entry in the `fac` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rep {
  /// Field representing the struct ID.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// Field representing the first faction ID.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionID1")]
  pub faction_id1: Option<NwValue<u32>>,

  /// Field representing the second faction ID.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionID2")]
  pub faction_id2: Option<NwValue<u32>>,

  /// Field representing the faction reputation value.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "FactionRep")]
  pub faction_rep: Option<NwValue<u32>>,
}
