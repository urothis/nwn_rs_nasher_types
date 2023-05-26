//! # Area
//! Structs for the `are` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

/// Area struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChanceLightning")]
    pub chance_lightning: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChanceRain")]
    pub chance_rain: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ChanceSnow")]
    pub chance_snow: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comments")]
    pub comments: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Creator_ID")]
    pub creator_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DayNightCycle")]
    pub day_night_cycle: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Expansion_List")]
    pub expansion_list: Option<NwValue<Vec<String>>>, // Assuming string data in list
    #[serde(skip_serializing_if = "Option::is_none", rename = "Flags")]
    pub flags: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FogClipDist")]
    pub fog_clip_dist: Option<NwValue<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Height")]
    pub height: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsNight")]
    pub is_night: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LightingScheme")]
    pub lighting_scheme: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
    pub load_screen_id: Option<NwValue<u16>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ModListenCheck")]
    pub mod_listen_check: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ModSpotCheck")]
    pub mod_spot_check: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonAmbientColor")]
    pub moon_ambient_color: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonDiffuseColor")]
    pub moon_diffuse_color: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonFogAmount")]
    pub moon_fog_amount: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonFogColor")]
    pub moon_fog_color: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "MoonShadows")]
    pub moon_shadows: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
    pub name: Option<NwValue<super::LocalizedText>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "NoRest")]
    pub no_rest: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnEnter")]
    pub on_enter: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnExit")]
    pub on_exit: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
    pub on_heartbeat: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
    pub on_user_defined: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PlayerVsPlayer")]
    pub player_vs_player: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ResRef")]
    pub res_ref: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ShadowOpacity")]
    pub shadow_opacity: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SkyBox")]
    pub sky_box: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunAmbientColor")]
    pub sun_ambient_color: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunDiffuseColor")]
    pub sun_diffuse_color: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunFogAmount")]
    pub sun_fog_amount: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunFogColor")]
    pub sun_fog_color: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "SunShadows")]
    pub sun_shadows: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tag")]
    pub tag: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_List")]
    pub tile_list: Option<NwValue<Vec<Tile>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tileset")]
    pub tileset: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Width")]
    pub width: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "WindPower")]
    pub wind_power: Option<NwValue<i32>>,
}

/// Area Tile struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    #[serde(rename = "__struct_id")]
    pub struct_id: u8,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_AnimLoop1")]
    pub tile_anim_loop1: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_AnimLoop2")]
    pub tile_anim_loop2: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_AnimLoop3")]
    pub tile_anim_loop3: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_Height")]
    pub tile_height: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_ID")]
    pub tile_id: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_MainLight1")]
    pub tile_main_light1: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_MainLight2")]
    pub tile_main_light2: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_Orientation")]
    pub tile_orientation: Option<NwValue<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_SrcLight1")]
    pub tile_src_light1: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Tile_SrcLight2")]
    pub tile_src_light2: Option<NwValue<u8>>,
}
