//! # Placeable
//! Structs for the `utp` file format

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utp {
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimationState")]
    pub animation_state: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "BodyBag")]
    pub body_bag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
    pub close_lock_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Conversation")]
    pub conversation: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
    pub current_hp: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Fort")]
    pub fortitude: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hardness")]
    pub hardness: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HasInventory")]
    pub has_inventory: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HP")]
    pub hp: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interruptable")]
    pub interruptable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyRequired")]
    pub key_required: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Lockable")]
    pub lockable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Locked")]
    pub locked: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClosed")]
    pub on_closed: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
    pub on_damaged: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDeath")]
    pub on_death: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnInvDisturbed")]
    pub on_inv_disturbed: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnLock")]
    pub on_lock: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
    pub on_melee_attacked: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpen")]
    pub on_open: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
    pub on_spell_cast_at: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
    pub on_unlock: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUsed")]
    pub on_used: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
    pub open_lock_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Ref")]
    pub reflex: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Static")]
    pub _static: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectable")]
    pub trap_detectable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
    pub trap_detect_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapDisarmable")]
    pub trap_disarmable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
    pub trap_flag: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapOneShot")]
    pub trap_one_shot: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TrapType")]
    pub trap_type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    pub _type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Useable")]
    pub usable: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Will")]
    pub will: Option<NwValue<u8>>,
}
