use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Utm {
    #[serde(skip_serializing_if = "Option::is_none", rename = "BlackMarket")]
    pub black_market: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "BM_MarkDown")]
    pub bm_mark_down: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LocName")]
    pub loc_name: Option<NwValue<LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MarkDown")]
    pub mark_down: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MarkUp")]
    pub mark_up: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnOpenStore")]
    pub on_open_store: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "StoreList")]
    pub store_list: Option<NwValue<Vec<Store>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ItemList")]
    pub item_list: Option<NwValue<Vec<Item>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Infinite")]
    pub infinite: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "InventoryRes")]
    pub inventory_res: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_PosX")]
    pub repos_pos_x: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Repos_Posy")]
    pub repos_posy: Option<NwValue<u32>>,
}
