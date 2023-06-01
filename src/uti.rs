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

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_Belt")]
  pub armor_part_belt: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LBicep")]
  pub armor_part_lbicep: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LFArm")]
  pub armor_part_lfarm: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LFoot")]
  pub armor_part_lfoot: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LHand")]
  pub armor_part_lhand: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LShin")]
  pub armor_part_lshin: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LShoul")]
  pub armor_part_lshoul: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_LThigh")]
  pub armor_part_lthigh: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_Neck")]
  pub armor_part_neck: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_Pelvis")]
  pub armor_part_pelvis: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RBicep")]
  pub armor_part_rbicep: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RFArm")]
  pub armor_part_rfarm: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RFoot")]
  pub armor_part_rfoot: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RHand")]
  pub armor_part_rhand: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_Robe")]
  pub armor_part_robe: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RShin")]
  pub armor_part_rshin: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RShoul")]
  pub armor_part_rshoul: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_RThigh")]
  pub armor_part_rthigh: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ArmorPart_Torso")]
  pub armor_part_torso: Option<NwValue<u8>>,

  /// The base item of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "BaseItem")]
  pub base_item: Option<NwValue<i32>>,

  /// The number of charges the item has.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Charges")]
  pub charges: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cloth1Color")]
  pub cloth1_color: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cloth2Color")]
  pub cloth2_color: Option<NwValue<u8>>,

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

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Leather1Color")]
  pub leather1_color: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Leather2Color")]
  pub leather2_color: Option<NwValue<u8>>,

  /// The localized name of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
  pub localized_name: Option<NwValue<LocalizedName>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Metal1Color")]
  pub metal1_color: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Metal2Color")]
  pub metal2_color: Option<NwValue<u8>>,

  /// The model part 1 of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ModelPart1")]
  pub model_part1: Option<NwValue<u8>>,

  /// The model part 2 of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ModelPart2")]
  pub model_part2: Option<NwValue<u8>>,

  /// The model part 3 of the item.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ModelPart3")]
  pub model_part3: Option<NwValue<u8>>,

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

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "VarTable")]
  pub var_table: Option<NwValue<Vec<Variable>>>,
}

/// A struct that represents a localized string.
/// Determined by the players's language setting.
/// <https://nwnlexicon.com/index.php?title=PLAYER_LANGUAGE>
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedName {
  /// The English version of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "0")]
  pub english: Option<String>,

  /// The French version of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "1")]
  pub french: Option<String>,

  /// The German version of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "2")]
  pub german: Option<String>,

  /// The Italian version of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "3")]
  pub italian: Option<String>,

  /// The Spanish version of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "4")]
  pub spanish: Option<String>,

  /// The Polish version of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "5")]
  pub polish: Option<String>,

  /// The id of the string.
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "id")]
  pub id: Option<u32>,
}

/// Represents a property of an Item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
  ///
  #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ChanceAppear")]
  pub chance_appear: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CostTable")]
  pub cost_table: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CostValue")]
  pub cost_value: Option<NwValue<u16>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Param1")]
  pub param1: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Param1Value")]
  pub param1_value: Option<NwValue<u8>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PropertyName")]
  pub property_name: Option<NwValue<u16>>,

  ///
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Subtype")]
  pub subtype: Option<NwValue<u16>>,
}
