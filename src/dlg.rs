//! # Dialog
//! Structs for the `dlg` file format

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dlg {
    #[serde(skip_serializing_if = "Option::is_none", rename = "DelayEntry")]
    pub delay_entry: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DelayReply")]
    pub delay_reply: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EndConverAbort")]
    pub end_conver_abort: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EndConversation")]
    pub end_conversation: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntryList")]
    pub entry_list: Option<NwValue<Vec<Entry>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "NumWords")]
    pub num_words: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreventZoomIn")]
    pub prevent_zoom_in: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ReplyList")]
    pub reply_list: Option<NwValue<Vec<ReplyEntry>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "StartingList")]
    pub starting_list: Option<NwValue<Vec<StartingList>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Animation")]
    pub animation: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimLoop")]
    pub anim_loop: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Delay")]
    pub delay: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Quest")]
    pub quest: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "QuestEntry")]
    pub quest_entry: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RepliesList")]
    pub replies_list: Option<NwValue<Vec<Reply>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntriesList")]
    pub entries_list: Option<NwValue<Vec<EntryReply>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Script")]
    pub script: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sound")]
    pub sound: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Speaker")]
    pub speaker: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Text")]
    pub text: Option<NwValue<Text>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyEntry {
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Animation")]
    pub animation: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimLoop")]
    pub anim_loop: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Delay")]
    pub delay: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntriesList")]
    pub entries_list: Option<NwValue<Vec<EntryReply>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Quest")]
    pub quest: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "QuestEntry")]
    pub quest_entry: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "RepliesList")]
    pub replies_list: Option<NwValue<Vec<Reply>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Script")]
    pub script: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sound")]
    pub sound: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Speaker")]
    pub speaker: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Text")]
    pub text: Option<NwValue<Text>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryReply {
    #[serde(rename = "__struct_id")]
    pub struct_id: u8,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Index")]
    pub index: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsChild")]
    pub is_child: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}

//
#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    #[serde(rename = "__struct_id")]
    pub struct_id: u8,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Index")]
    pub index: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsChild")]
    pub is_child: Option<NwValue<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartingList {
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Index")]
    pub index: Option<NwValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}
