//! # Module information
//! Structs for the `ifo` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ifo {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Expansion_Pack")]
    pub expansion_pack: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Area_list")]
    pub mod_area_list: Option<NwValue<Vec<Area>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_CacheNSSList")]
    pub mod_cache_nss_list: Option<NwValue<Vec<NSS>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Creator_ID")]
    pub mod_creator_id: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_CustomTlk")]
    pub mod_custom_tlk: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_CutSceneList")]
    pub mod_cut_scene_list: Option<NwValue<Vec<CutScene>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_DawnHour")]
    pub mod_dawn_hour: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Description")]
    pub mod_description: Option<NwValue<super::LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_DuskHour")]
    pub mod_dusk_hour: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Entry_Area")]
    pub mod_entry_area: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Entry_Dir_X")]
    pub mod_entry_dir_x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Entry_Dir_Y")]
    pub mod_entry_dir_y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Entry_X")]
    pub mod_entry_x: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Entry_Y")]
    pub mod_entry_y: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Entry_Z")]
    pub mod_entry_z: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Expan_List")]
    pub mod_expan_list: Option<NwValue<Vec<Expansion>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_GVar_List")]
    pub mod_g_var_list: Option<NwValue<Vec<GlobalVariable>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_HakList")]
    pub mod_hak_list: Option<NwValue<Vec<Hak>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_IsSaveGame")]
    pub mod_is_save_game: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_MinGameVer")]
    pub mod_min_game_ver: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_MinPerHour")]
    pub mod_min_per_hour: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Name")]
    pub mod_name: Option<NwValue<super::LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnAcquirItem")]
    pub mod_on_acquir_item: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnActvtItem")]
    pub mod_on_actvt_item: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnClientEntr")]
    pub mod_on_client_entr: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnClientLeav")]
    pub mod_on_client_leav: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnCutsnAbort")]
    pub mod_on_cutsn_abort: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnHeartbeat")]
    pub mod_on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnModLoad")]
    pub mod_on_mod_load: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnModStart")]
    pub mod_on_mod_start: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrChat")]
    pub mod_on_plr_chat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrDeath")]
    pub mod_on_plr_death: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrDying")]
    pub mod_on_plr_dying: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrEqItm")]
    pub mod_on_plr_eq_itm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrLvlUp")]
    pub mod_on_plr_lvl_up: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrRest")]
    pub mod_on_plr_rest: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnPlrUnEqItm")]
    pub mod_on_plr_un_eq_itm: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnSpawnBtnDn")]
    pub mod_on_spawn_btn_dn: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnUnAqreItem")]
    pub mod_on_un_aqre_item: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_OnUsrDefined")]
    pub mod_on_usr_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_StartDay")]
    pub mod_start_day: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_StartHour")]
    pub mod_start_hour: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_StartMonth")]
    pub mod_start_month: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_StartMovie")]
    pub mod_start_movie: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_StartYear")]
    pub mod_start_year: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Tag")]
    pub mod_tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Version")]
    pub mod_version: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_XPScale")]
    pub mod_x_p_scale: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "VarTable")]
    pub var_table: Option<NwValue<Vec<Variable>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Area_Name")]
    pub name: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NSS {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CutScene {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expansion {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalVariable {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hak {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Mod_Hak")]
    pub value: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub name: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    pub _type: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Value")]
    pub value: Option<NwValue<Value>>,
}
