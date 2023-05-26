//! # Palette
//! Structs for the `itp` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Itp {
    #[serde(rename = "MAIN")]
    pub entry: Option<NwValue<Vec<Entry>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "CR")]
    pub cr: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FACTION")]
    pub faction: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LIST")]
    pub list: Option<NwValue<Vec<Entry>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "NAME")]
    pub name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RESREF")]
    pub res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "STRREF")]
    pub string_ref: Option<NwValue<u32>>,
}
