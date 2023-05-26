//! # Sound
//! Structs for the `uts` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Uts {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Continuous")]
    pub continuous: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Elevation")]
    pub elevation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Hours")]
    pub hours: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Interval")]
    pub interval: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "IntervalVrtn")]
    pub interval_vrtn: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Looping")]
    pub looping: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MaxDistance")]
    pub max_distance: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MinDistance")]
    pub min_distance: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PaletteID")]
    pub palette_id: Option<NwValue<i16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PitchVariation")]
    pub pitch_variation: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Positional")]
    pub positional: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
    pub priority: Option<NwValue<i16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Random")]
    pub random: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RandomPosition")]
    pub random_position: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RandomRangeX")]
    pub random_range_x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RandomRangeY")]
    pub random_range_y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sounds")]
    pub sounds: Option<NwValue<Vec<Sound>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
    pub template_res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Times")]
    pub times: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Volume")]
    pub volume: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "VolumeVrtn")]
    pub volume_vrtn: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sound")]
    pub sound: Option<NwValue<String>>,
}
