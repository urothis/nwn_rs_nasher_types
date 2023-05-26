//! # Journal
//! Structs for the `jrl` file format

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jrl {
    #[serde(rename = "Categories")]
    pub categories: Option<NwValue<Vec<Category>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntryList")]
    pub entry_list: Option<NwValue<Vec<Entry>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Picture")]
    pub picture: Option<NwValue<u64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Priority")]
    pub priority: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "XP")]
    pub xp: Option<NwValue<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "End")]
    pub end: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Text")]
    pub text: Option<NwValue<LocalizedText>>,
}
