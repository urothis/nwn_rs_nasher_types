pub mod are;
pub mod dlg;
pub mod fac;
pub mod gic;
pub mod git;
pub mod ifo;
pub mod itp;
pub mod jrl;
pub mod utc;
pub mod utd;
pub mod ute;
pub mod uti;
pub mod utm;
pub mod utp;
pub mod uts;
pub mod utt;
pub mod utw;

use serde::{Deserialize, Serialize};

/// A library for reading and writing Neverwinter Nights module files generated from Nasher https://github.com/squattingmonk/nasher
/// The goal is this library is to allow for the reading and writing of these json files in such a fashion as to not replace nasher, but to hook into it for building.
/// This should allow for the creation of a Rust based module builder that can be used to create modules for Neverwinter Nights.

#[derive(Serialize, Deserialize)]
#[serde(tag = "__data_type")]
pub enum NwType {
    #[serde(alias = "ARE ", rename = "ARE ")]
    Area(are::Are),
    #[serde(alias = "DLG ", rename = "DLG ")]
    Dialog(dlg::Dlg),
    #[serde(alias = "FAC ", rename = "FAC ")]
    Faction(fac::Fac),
    #[serde(alias = "GIC ", rename = "GIC ")]
    AreaComments(gic::Gic),
    #[serde(alias = "GIT ", rename = "GIT ")]
    AreaInfo(git::Git),
    #[serde(alias = "IFO ", rename = "IFO ")]
    ModuleInfo(ifo::Ifo),
    #[serde(alias = "ITP ", rename = "ITP ")]
    Palette(itp::Itp),
    #[serde(alias = "JRL ", rename = "JRL ")]
    Journal(jrl::Jrl),
    #[serde(alias = "UTC ", rename = "UTC ")]
    Creature(utc::Utc),
    #[serde(alias = "UTD ", rename = "UTD ")]
    Door(utd::Utd),
    #[serde(alias = "UTE ", rename = "UTE ")]
    Encounter(ute::Ute),
    #[serde(alias = "UTI ", rename = "UTI ")]
    ItemBlueprint(uti::Uti),
    #[serde(alias = "UTM ", rename = "UTM ")]
    Store(utm::Utm),
    #[serde(alias = "UTP ", rename = "UTP ")]
    Placeable(utp::Utp),
    #[serde(alias = "UTS ", rename = "UTS ")]
    Sound(uts::Uts),
    #[serde(alias = "UTT ", rename = "UTT ")]
    Trigger(utt::Utt),
    #[serde(alias = "UTW ", rename = "UTW ")]
    Waypoint(utw::Utw),
}

impl NwType {
    pub fn get_type(&self) -> String {
        match self {
            NwType::Area(_) => "ARE".to_string(),
            NwType::Dialog(_) => "DLG".to_string(),
            NwType::Faction(_) => "FAC".to_string(),
            NwType::AreaComments(_) => "GIC".to_string(),
            NwType::AreaInfo(_) => "GIT".to_string(),
            NwType::ModuleInfo(_) => "IFO".to_string(),
            NwType::Palette(_) => "ITP".to_string(),
            NwType::Journal(_) => "JRL".to_string(),
            NwType::Creature(_) => "UTC".to_string(),
            NwType::Door(_) => "UTD".to_string(),
            NwType::Encounter(_) => "UTE".to_string(),
            NwType::ItemBlueprint(_) => "UTI".to_string(),
            NwType::Store(_) => "UTM".to_string(),
            NwType::Placeable(_) => "UTP".to_string(),
            NwType::Sound(_) => "UTS".to_string(),
            NwType::Trigger(_) => "UTT".to_string(),
            NwType::Waypoint(_) => "UTW".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NwValue<T> {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "value")]
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NwStruct<T> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "value")]
    pub value: T,
}

/// A struct that represents a localized string.
/// Determined by the players's language setting.
/// https://nwnlexicon.com/index.php?title=PLAYER_LANGUAGE
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedText {
    #[serde(skip_serializing_if = "Option::is_none", rename = "0")]
    pub english: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "1")]
    pub french: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "2")]
    pub german: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "3")]
    pub italian: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "4")]
    pub spanish: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "5")]
    pub polish: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "id")]
    pub id: Option<u32>,
}