//! # Placeable
//! Structs for the `utp` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents a placeable in the `utp` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Utp {
  /// Animation state of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AnimationState")]
  pub animation_state: Option<NwValue<u8>>,

  /// Appearance of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Appearance")]
  pub appearance: Option<NwValue<u32>>,

  /// Key required to automatically remove the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey", with = "bool_as_u8")]
  pub auto_remove_key: Option<NwValue<bool>>,

  /// Whether the placeable is a body bag
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "BodyBag", with = "bool_as_u8")]
  pub body_bag: Option<NwValue<bool>>,

  /// Difficulty class for closing and locking the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
  pub close_lock_dc: Option<NwValue<u8>>,

  /// Comment associated with the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Comment")]
  pub comment: Option<NwValue<String>>,

  /// Conversation script associated with the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Conversation")]
  pub conversation: Option<NwValue<String>>,

  /// Current hit points of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
  pub current_hp: Option<NwValue<i16>>,

  /// Description of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Description")]
  pub description: Option<NwValue<LocalizedText>>,

  /// Difficulty class for disarming the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
  pub disarm_dc: Option<NwValue<u8>>,

  /// Faction of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Faction")]
  pub faction: Option<NwValue<u32>>,

  /// Fortitude saving throw modifier for the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Fort")]
  pub fortitude: Option<NwValue<u8>>,

  /// Hardness of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Hardness")]
  pub hardness: Option<NwValue<u8>>,

  /// Whether the placeable has inventory
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HasInventory", with = "bool_as_u8")]
  pub has_inventory: Option<NwValue<bool>>,

  /// Hit points of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HP")]
  pub hp: Option<NwValue<i16>>,

  /// Whether the placeable is interruptable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Interruptable", with = "bool_as_u8")]
  pub interruptable: Option<NwValue<bool>>,

  /// Key name required to interact with the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyName")]
  pub key_name: Option<NwValue<String>>,

  /// Whether a key is required to interact with the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyRequired", with = "bool_as_u8")]
  pub key_required: Option<NwValue<bool>>,

  /// Whether the placeable is lockable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Lockable", with = "bool_as_u8")]
  pub lockable: Option<NwValue<bool>>,

  /// Whether the placeable is locked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Locked", with = "bool_as_u8")]
  pub locked: Option<NwValue<bool>>,

  /// Localized name of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocName")]
  pub loc_name: Option<NwValue<LocalizedText>>,

  /// Script to run when the placeable is closed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnClosed")]
  pub on_closed: Option<NwValue<String>>,

  /// Script to run when the placeable is damaged
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
  pub on_damaged: Option<NwValue<String>>,

  /// Script to run when the placeable is destroyed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDeath")]
  pub on_death: Option<NwValue<String>>,

  /// Script to run when the placeable is disarmed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
  pub on_disarm: Option<NwValue<String>>,

  /// Script to run on the heartbeat of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
  pub on_heartbeat: Option<NwValue<String>>,

  /// Script to run when the placeable's inventory is disturbed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnInvDisturbed")]
  pub on_inv_disturbed: Option<NwValue<String>>,

  /// Script to run when the placeable is locked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnLock")]
  pub on_lock: Option<NwValue<String>>,

  /// Script to run when the placeable is attacked in melee
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
  pub on_melee_attacked: Option<NwValue<String>>,

  /// Script to run when the placeable is opened
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnOpen")]
  pub on_open: Option<NwValue<String>>,

  /// Script to run when a spell is cast at the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
  pub on_spell_cast_at: Option<NwValue<String>>,

  /// Script to run when a trap is triggered on the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
  pub on_trap_triggered: Option<NwValue<String>>,

  /// Script to run when the placeable is unlocked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
  pub on_unlock: Option<NwValue<String>>,

  /// Script to run when the placeable is used
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUsed")]
  pub on_used: Option<NwValue<String>>,

  /// Script to run when the placeable has a user-defined event
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
  pub on_user_defined: Option<NwValue<String>>,

  /// Difficulty class for opening the lock on the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
  pub open_lock_dc: Option<NwValue<u8>>,

  /// ID of the palette for the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PaletteID")]
  pub palette_id: Option<NwValue<u8>>,

  /// Whether the placeable is part of a plot
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Plot", with = "bool_as_u8")]
  pub plot: Option<NwValue<bool>>,

  /// ID of the portrait for the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PortraitId")]
  pub portrait_id: Option<NwValue<u16>>,

  /// Reflex save difficulty class for the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Ref")]
  pub reflex: Option<NwValue<u8>>,

  /// Whether the placeable is static
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Static", with = "bool_as_u8")]
  pub _static: Option<NwValue<bool>>,

  /// Tag of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// Resref of the template for the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,

  /// Whether the trap on the placeable is detectable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectable", with = "bool_as_u8")]
  pub trap_detectable: Option<NwValue<bool>>,

  /// Difficulty class for detecting the trap on the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
  pub trap_detect_dc: Option<NwValue<u8>>,

  /// Whether the trap on the placeable is disarmable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDisarmable", with = "bool_as_u8")]
  pub trap_disarmable: Option<NwValue<bool>>,

  /// Flag indicating the trap status of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapFlag", with = "bool_as_u8")]
  pub trap_flag: Option<NwValue<bool>>,

  /// Whether the trap on the placeable is one-shot
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapOneShot", with = "bool_as_u8")]
  pub trap_one_shot: Option<NwValue<bool>>,

  /// Type of the trap on the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapType")]
  pub trap_type: Option<NwValue<u8>>,

  /// Type of the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Type")]
  pub _type: Option<NwValue<u8>>,

  /// Whether the placeable is usable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Useable", with = "bool_as_u8")]
  pub usable: Option<NwValue<bool>>,

  /// Will save difficulty class for the placeable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Will")]
  pub will: Option<NwValue<u8>>,
}
