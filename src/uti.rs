//! # Item
//! Structs for the `uti` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents an Item in the `uti` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Uti {
  /// The additional cost of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AddCost")]
  pub add_cost: Option<NwValue<u32>>,

  /// The base item of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "BaseItem")]
  pub base_item: Option<NwValue<i32>>,

  /// The number of charges the item has.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Charges")]
  pub charges: Option<NwValue<u8>>,

  /// Comment for the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Comment")]
  pub comment: Option<NwValue<String>>,

  /// The cost of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cost")]
  pub cost: Option<NwValue<u32>>,

  /// Indicates if the item is cursed.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cursed", with = "bool_as_u8")]
  pub cursed: Option<NwValue<bool>>,

  /// The localized identified description of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "DescIdentified")]
  pub desc_identified: Option<NwValue<LocalizedText>>,

  /// The localized description of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Description")]
  pub description: Option<NwValue<LocalizedText>>,

  /// Indicates if the item is identified.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Identified")]
  pub identified: Option<NwValue<i32>>,

  /// The localized name of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
  pub localized_name: Option<NwValue<LocalizedText>>,

  /// The model part 1 of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ModelPart1")]
  pub model_part1: Option<NwValue<u8>>,

  /// The palette ID of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PaletteID")]
  pub palette_id: Option<NwValue<u8>>,

  /// Indicates if the item is a plot item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Plot", with = "bool_as_u8")]
  pub plot: Option<NwValue<bool>>,

  /// The list of properties of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PropertiesList")]
  pub properties_list: Option<NwValue<Vec<Property>>>,

  /// The stack size of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "StackSize")]
  pub stack_size: Option<NwValue<u16>>,

  /// Indicates if the item is stolen.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Stolen", with = "bool_as_u8")]
  pub stolen: Option<NwValue<bool>>,

  /// The tag of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// The template resource reference (ResRef) of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,
}

/// Represents a property of an Item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
  /// The unique identifier of the property.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,
}
