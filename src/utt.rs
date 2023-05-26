//! # Trigger
//! Structs for the `utt` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utt {
    #[serde(skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey")]
    pub auto_remove_key: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Cursor")]
    pub cursor: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
    pub disarm_dc: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Faction")]
    pub faction: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HighlightHeight")]
    pub highlight_height: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "KeyName")]
    pub key_name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedToFlags")]
    pub linked_to_flags: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
    pub load_screen_id: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnClick")]
    pub on_click: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
    pub on_disarm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
    pub on_trap_triggered: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PortraitId")]
    pub portrait_id: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptHeartbeat")]
    pub script_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnEnter")]
    pub script_on_enter: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptOnExit")]
    pub script_on_exit: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ScriptUserDefine")]
    pub script_user_define: Option<NwValue<String>>,
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
}
