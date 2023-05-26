use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utw {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Appearance")]
    pub appearance: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "HasMapNote")]
    pub has_map_note: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
    pub linked_to: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MapNote")]
    pub map_note: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MapNoteEnabled")]
    pub map_note_enabled: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
}
