//! # Item
//! Structs for the `uti` file format

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Uti {
    #[serde(skip_serializing_if = "Option::is_none", rename = "AddCost")]
    pub add_cost: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "BaseItem")]
    pub base_item: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Charges")]
    pub charges: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Cost")]
    pub cost: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Cursed")]
    pub cursed: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DescIdentified")]
    pub desc_identified: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Description")]
    pub description: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Identified")]
    pub identified: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
    pub localized_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ModelPart1")]
    pub model_part1: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Plot")]
    pub plot: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PropertiesList")]
    pub properties_list: Option<NwValue<Vec<Property>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "StackSize")]
    pub stack_size: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Stolen")]
    pub stolen: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
}
