//! # Area information
//! Structs for the `git` file format

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::*;

/// Area information
#[derive(Debug, Serialize, Deserialize)]
pub struct Git {
  /// Area properties (serialized as "AreaProperties")
  #[serde(rename = "AreaProperties")]
  pub area_properties: NwStruct<AreaProperty>,

  /// List of creatures (serialized as "Creature List")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Creature List")]
  pub creature_list: Option<NwValue<Vec<Creature>>>,

  /// List of doors (serialized as "Door List")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Door List")]
  pub door_list: Option<NwValue<Vec<Door>>>,

  /// List of encounters (serialized as "Encounter List")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Encounter List")]
  pub encounter_list: Option<NwValue<Vec<Encounter>>>,

  /// List (serialized as "List")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "List")]
  pub list: Option<NwValue<Vec<List>>>,

  /// List of placeables (serialized as "Placeable List")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Placeable List")]
  pub placeable_list: Option<NwValue<Vec<Placeable>>>,

  /// List of sounds (serialized as "SoundList")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "SoundList")]
  pub sound_list: Option<NwValue<Vec<Sound>>>,

  /// List of stores (serialized as "StoreList")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "StoreList")]
  pub store_list: Option<NwValue<Vec<Store>>>,

  /// List of triggers (serialized as "TriggerList")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TriggerList")]
  pub trigger_list: Option<NwValue<Vec<Trigger>>>,

  /// List of waypoints (serialized as "WaypointList")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "WaypointList")]
  pub waypoint_list: Option<NwValue<Vec<Waypoint>>>,
}

/// Area properties
#[derive(Debug, Serialize, Deserialize)]
pub struct AreaProperty {
  /// Unique identifier for the struct (serialized as "__struct_id")
  #[serde(rename = "__struct_id")]
  pub struct_id: u32,

  /// Daytime ambient sound (serialized as "AmbientSndDay")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AmbientSndDay")]
  pub ambient_snd_day: Option<NwValue<i32>>,

  /// Volume for daytime ambient sound (serialized as "AmbientSndDayVol")
  #[serde(
    default, skip_serializing_if = "Option::is_none",
    rename = "AmbientSndDayVol"
  )]
  pub ambient_snd_day_vol: Option<NwValue<i32>>,

  /// Nighttime ambient sound (serialized as "AmbientSndNight")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AmbientSndNight")]
  pub ambient_snd_night: Option<NwValue<i32>>,

  /// Volume for nighttime ambient sound (serialized as "AmbientSndNitVol")
  #[serde(
    default, skip_serializing_if = "Option::is_none",
    rename = "AmbientSndNitVol"
  )]
  pub ambient_snd_nit_vol: Option<NwValue<i32>>,

  /// Environmental audio (serialized as "EnvAudio")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "EnvAudio")]
  pub env_audio: Option<NwValue<i32>>,

  /// Battle music (serialized as "MusicBattle")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MusicBattle")]
  pub music_battle: Option<NwValue<i32>>,

  /// Daytime music (serialized as "MusicDay")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MusicDay")]
  pub music_day: Option<NwValue<i32>>,

  /// Music delay (serialized as "MusicDelay")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MusicDelay")]
  pub music_delay: Option<NwValue<i32>>,

  /// Nighttime music (serialized as "MusicNight")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MusicNight")]
  pub music_night: Option<NwValue<i32>>,
}

/// TODO
/// Creature
#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {}

/// Door
#[derive(Debug, Serialize, Deserialize)]
pub struct Door {
  /// The unique identifier for the door
  #[serde(rename = "__struct_id")]
  pub struct_id: u32,

  /// The animation state of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AnimationState")]
  pub animation_state: Option<NwValue<u8>>,

  /// The appearance of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Appearance")]
  pub appearance: Option<NwValue<u32>>,

  /// The auto-remove key of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey", with = "bool_as_u8")]
  pub auto_remove_key: Option<NwValue<bool>>,

  /// The bearing of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Bearing")]
  pub bearing: Option<NwValue<Decimal>>,

  /// The close lock DC of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
  pub close_lock_dc: Option<NwValue<u8>>,

  /// The comment of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Comment")]
  pub comment: Option<NwValue<String>>,

  /// The conversation of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Conversation")]
  pub conversation: Option<NwValue<String>>,

  /// The current HP (hit points) of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
  pub current_hp: Option<NwValue<i16>>,

  /// The description of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Description")]
  pub description: Option<NwValue<LocalizedText>>,

  /// The disarm DC of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
  pub disarm_dc: Option<NwValue<u8>>,

  /// The faction of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Faction")]
  pub faction: Option<NwValue<u32>>,

  /// The fortitude of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Fort")]
  pub fortitude: Option<NwValue<u8>>,

  /// The generic type (new) of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "GenericType_New")]
  pub generic_type_new: Option<NwValue<u32>>,

  /// The hardness of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Hardness")]
  pub hardness: Option<NwValue<u8>>,

  /// The HP (hit points) of the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HP")]
  pub hp: Option<NwValue<i16>>,

  /// Represents whether the door can be interrupted
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Interruptable", with = "bool_as_u8")]
  pub interuptable: Option<NwValue<bool>>,

  /// Represents the name of the key required to open the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyName")]
  pub key_name: Option<NwValue<String>>,

  /// Represents whether a key is required to open the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyRequired", with = "bool_as_u8")]
  pub key_required: Option<NwValue<bool>>,

  /// Represents the door linked to this one
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
  pub linked_to: Option<NwValue<String>>,

  /// Represents the flags associated with the linked door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LinkedToFlags")]
  pub linked_to_flags: Option<NwValue<u8>>,

  /// Represents the ID of the load screen associated with the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
  pub load_screen_id: Option<NwValue<u16>>,

  /// Represents whether the door is lockable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Lockable", with = "bool_as_u8")]
  pub lockable: Option<NwValue<bool>>,

  /// Represents whether the door is locked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Locked", with = "bool_as_u8")]
  pub locked: Option<NwValue<bool>>,

  /// Represents the localized name of the door's location
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocName")]
  pub location_name: Option<NwValue<LocalizedText>>,

  /// Represents the action to perform when the door is clicked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnClick")]
  pub on_click: Option<NwValue<String>>,

  /// Represents the action to perform when the door is closed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnClosed")]
  pub on_closed: Option<NwValue<String>>,

  /// Represents the action to perform when the door is damaged
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
  pub on_damaged: Option<NwValue<String>>,

  /// Represents the action to perform when the door is destroyed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDeath")]
  pub on_death: Option<NwValue<String>>,

  /// Represents the action to perform when the door is disarmed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
  pub on_disarm: Option<NwValue<String>>,

  /// Represents the action to perform when failed to open the door
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnFailToOpen")]
  pub on_fail_to_open: Option<NwValue<String>>,

  /// Represents the action to perform on door heartbeat
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
  pub on_heartbeat: Option<NwValue<String>>,

  /// Represents the action to perform when the door is locked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnLock")]
  pub on_lock: Option<NwValue<String>>,

  /// Represents the action to perform when the object is melee attacked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
  pub on_melee_attacked: Option<NwValue<String>>,

  /// Represents the action to perform when the object is opened
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnOpen")]
  pub on_open: Option<NwValue<String>>,

  /// Represents the action to perform when a spell is cast at the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
  pub on_spell_cast_at: Option<NwValue<String>>,

  /// Represents the action to perform when the trap associated with the object is triggered
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
  pub on_trap_triggered: Option<NwValue<String>>,

  /// Represents the action to perform when the object is unlocked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
  pub on_unlock: Option<NwValue<String>>,

  /// Represents the action to perform as defined by the user
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
  pub on_user_defined: Option<NwValue<String>>,

  /// Represents the difficulty class (DC) to open a lock on the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
  pub open_lock_dc: Option<NwValue<u8>>,

  /// Represents the plot associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Plot", with = "bool_as_u8")]
  pub plot: Option<NwValue<bool>>,

  /// Represents the ID of the portrait associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PortraitId")]
  pub portrait_id: Option<NwValue<u16>>,

  /// Represents the reference ID of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Ref")]
  pub ref_: Option<NwValue<u8>>,

  /// Represents the tag of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// Represents the template resource reference associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,

  /// Represents whether the trap associated with the object is detectable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectable", with = "bool_as_u8")]
  pub trap_detectable: Option<NwValue<bool>>,

  /// Represents the difficulty class (DC) to detect the trap associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectDC", with = "bool_as_u8")]
  pub trap_detect_dc: Option<NwValue<bool>>,

  /// Represents whether the trap associated with the object is disarmable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDisarmable", with = "bool_as_u8")]
  pub trap_disarmable: Option<NwValue<bool>>,

  /// Represents the flag associated with the trap
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
  pub trap_flag: Option<NwValue<u8>>,

  /// Represents whether the trap associated with the object is a one-shot trap
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapOneShot", with = "bool_as_u8")]
  pub trap_one_shot: Option<NwValue<bool>>,

  /// Represents the type of trap associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapType")]
  pub trap_type: Option<NwValue<u8>>,

  /// Represents the will attribute of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Will")]
  pub will: Option<NwValue<u8>>,

  /// Represents the x-coordinate of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "X")]
  pub x: Option<NwValue<Decimal>>,

  /// Represents the y-coordinate of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Y")]
  pub y: Option<NwValue<Decimal>>,

  /// Represents the z-coordinate of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Z")]
  pub z: Option<NwValue<Decimal>>,
}

/// Encounter
#[derive(Debug, Serialize, Deserialize)]
pub struct Encounter {
  /// Represents the struct ID of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// Represents whether the object is active
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Active", with = "bool_as_u8")]
  pub active: Option<NwValue<bool>>,

  /// Represents a comment associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Comment")]
  pub comment: Option<NwValue<String>>,

  /// Represents a list of creatures associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CreatureList")]
  pub creature_list: Option<NwValue<Vec<EncounterCreature>>>,

  /// Represents the difficulty level of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Difficulty")]
  pub difficulty: Option<NwValue<i32>>,

  /// Represents the difficulty index of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "DifficultyIndex")]
  pub difficulty_index: Option<NwValue<i32>>,

  /// Represents the faction ID associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Faction")]
  pub faction: Option<NwValue<u32>>,

  /// Represents the geometry information associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Geometry")]
  pub geometry: Option<NwValue<Vec<EncounterGeometry>>>,

  /// Represents the localized name of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
  pub localized_name: Option<NwValue<LocalizedText>>,

  /// Represents the maximum number of creatures associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MaxCreatures")]
  pub max_creatures: Option<NwValue<i32>>,

  /// Represents the script to execute when entering the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnEntered")]
  pub on_entered: Option<NwValue<String>>,

  /// Represents the script to execute when the object is exhausted
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnExhausted")]
  pub on_exhausted: Option<NwValue<String>>,

  /// Represents the script to execute when exiting the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnExit")]
  pub on_exit: Option<NwValue<String>>,

  /// Represents the script to execute on the object's heartbeat
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
  pub on_heartbeat: Option<NwValue<String>>,

  /// Represents the user-defined script associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
  pub on_user_defined: Option<NwValue<String>>,

  /// Represents whether the object is accessible only by players
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PlayerOnly")]
  pub player_only: Option<NwValue<u8>>,

  /// Represents the recommended number of creatures for the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "RecCreatures")]
  pub rec_creatures: Option<NwValue<i32>>,

  /// Represents whether the object is resettable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Reset", with = "bool_as_u8")]
  pub reset: Option<NwValue<bool>>,

  /// Represents the reset time of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ResetTime")]
  pub reset_time: Option<NwValue<i32>>,

  /// Represents the number of times the object respawns
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Respawns")]
  pub respawns: Option<NwValue<i32>>,

  /// Represents the spawn option for the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "SpawnOption")]
  pub spawn_option: Option<NwValue<i32>>,

  /// Represents the list of spawn points associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "SpawnPointList")]
  pub spawn_point_list: Option<NwValue<Vec<SpawnPoint>>>,

  /// Represents the tag associated with the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// Represents the template resource reference of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,

  /// Represents the X position of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "XPosition")]
  pub x_position: Option<NwValue<Decimal>>,

  /// Represents the Y position of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "YPosition")]
  pub y_position: Option<NwValue<Decimal>>,

  /// Represents the Z position of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ZPosition")]
  pub z_position: Option<NwValue<Decimal>>,
}

/// Encounter
#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterCreature {
  /// The unique identifier for the struct
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// The appearance value of the creature
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Appearance")]
  pub appearance: Option<NwValue<u32>>,

  /// A comment associated with the creature
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Comment")]
  pub comment: Option<NwValue<String>>,

  /// The challenge rating of the creature
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CR")]
  pub challenge_rating: Option<NwValue<Decimal>>,

  /// The resource reference of the creature
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ResRef")]
  pub res_ref: Option<NwValue<String>>,

  /// Indicates whether the creature should be single-spawned
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "SingleSpawn", with = "bool_as_u8")]
  pub single_spawn: Option<NwValue<bool>>,
}

/// Encounter geometry
#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterGeometry {
  /// The unique identifier for the struct
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// The X coordinate of the geometry
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "X")]
  pub x: Option<NwValue<Decimal>>,

  /// The Y coordinate of the geometry
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Y")]
  pub y: Option<NwValue<Decimal>>,

  /// The Z coordinate of the geometry
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Z")]
  pub z: Option<NwValue<Decimal>>,
}

/// TODO
/// List
#[derive(Debug, Serialize, Deserialize)]
pub struct List {}

/// Placeable
#[derive(Debug, Serialize, Deserialize)]
pub struct Placeable {
  /// Represents the struct ID of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// Represents the animation state of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AnimationState")]
  pub animation_state: Option<NwValue<u8>>,

  /// Represents the appearance of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Appearance")]
  pub appearance: Option<NwValue<u32>>,

  /// Represents the auto remove key of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey", with = "bool_as_u8")]
  pub auto_remove_key: Option<NwValue<bool>>,

  /// Represents the bearing of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Bearing")]
  pub bearing: Option<NwValue<Decimal>>,

  /// Represents the body bag of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "BodyBag")]
  pub body_bag: Option<NwValue<u8>>,

  /// Represents the close lock DC of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CloseLockDC")]
  pub close_lock_dc: Option<NwValue<u8>>,

  /// Represents the conversation of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Conversation")]
  pub conversation: Option<NwValue<String>>,

  /// Represents the current HP of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "CurrentHP")]
  pub current_hp: Option<NwValue<i16>>,

  /// Represents the description of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Description")]
  pub description: Option<NwValue<LocalizedText>>,

  /// Represents the disarm DC of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
  pub disarm_dc: Option<NwValue<u8>>,

  /// Represents the faction of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Faction")]
  pub faction: Option<NwValue<u32>>,

  /// Represents the fortitude of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Fort")]
  pub fortitude: Option<NwValue<u8>>,

  /// Represents the hardness of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Hardness")]
  pub hardness: Option<NwValue<u8>>,

  /// Represents whether the object has inventory
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HasInventory", with = "bool_as_u8")]
  pub has_inventory: Option<NwValue<bool>>,

  /// Represents the HP of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HP")]
  pub hp: Option<NwValue<i16>>,

  /// Represents whether the object is interruptible
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Interruptable", with = "bool_as_u8")]
  pub interuptable: Option<NwValue<bool>>,

  /// Represents the key name of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyName")]
  pub key_name: Option<NwValue<String>>,

  /// Represents whether a key is required for the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyRequired", with = "bool_as_u8")]
  pub key_required: Option<NwValue<bool>>,

  /// Represents whether the object is lockable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Lockable", with = "bool_as_u8")]
  pub lockable: Option<NwValue<bool>>,

  /// Represents whether the object is locked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Locked", with = "bool_as_u8")]
  pub locked: Option<NwValue<bool>>,

  /// Represents the location name of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocName")]
  pub location_name: Option<NwValue<LocalizedText>>,

  /// Represents the on click event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnClick")]
  pub on_click: Option<NwValue<String>>,

  /// Represents the on closed event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnClosed")]
  pub on_closed: Option<NwValue<String>>,

  /// Represents the on damaged event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDamaged")]
  pub on_damaged: Option<NwValue<String>>,

  /// Represents the on death event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDeath")]
  pub on_death: Option<NwValue<String>>,

  /// Represents the on disarm event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
  pub on_disarm: Option<NwValue<String>>,

  /// Represents the on heartbeat event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnHeartbeat")]
  pub on_heartbeat: Option<NwValue<String>>,

  /// Represents the on inventory disturbed event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnInvDisturbed")]
  pub on_inv_disturbed: Option<NwValue<String>>,

  /// Represents the on lock event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnLock")]
  pub on_lock: Option<NwValue<String>>,

  /// Represents the on melee attacked event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnMeleeAttacked")]
  pub on_melee_attacked: Option<NwValue<String>>,

  /// Represents the on open event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnOpen")]
  pub on_open: Option<NwValue<String>>,

  /// Represents the on spell cast at event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnSpellCastAt")]
  pub on_spell_cast_at: Option<NwValue<String>>,

  /// Represents the on trap triggered event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
  pub on_trap_triggered: Option<NwValue<String>>,

  /// Represents the on unlock event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUnlock")]
  pub on_unlock: Option<NwValue<String>>,

  /// Represents the on used event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUsed")]
  pub on_used: Option<NwValue<String>>,

  /// Represents the on user defined event of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnUserDefined")]
  pub on_user_defined: Option<NwValue<String>>,

  /// Represents the open lock DC (Difficulty Class) of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OpenLockDC")]
  pub open_lock_dc: Option<NwValue<u8>>,

  /// Represents whether the object is part of a plot
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Plot", with = "bool_as_u8")]
  pub plot: Option<NwValue<bool>>,

  /// Represents the portrait ID of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PortraitId")]
  pub portrait_id: Option<NwValue<u16>>,

  /// Represents the reflex saving throw of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Ref")]
  pub reflex: Option<NwValue<u8>>,

  /// Represents whether the object is static
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Static")]
  pub static_: Option<NwValue<u8>>,

  /// Represents the tag of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// Represents the template resource reference of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,

  /// Represents whether the trap is detectable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectable", with = "bool_as_u8")]
  pub trap_detectable: Option<NwValue<bool>>,

  /// Represents the trap detect DC (Difficulty Class) of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
  pub trap_detect_dc: Option<NwValue<u8>>,

  /// Represents whether the trap is disarmable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDisarmable", with = "bool_as_u8")]
  pub trap_disarmable: Option<NwValue<bool>>,

  /// Represents the trap flag of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
  pub trap_flag: Option<NwValue<u8>>,

  /// Represents whether the trap is a one-shot trap
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapOneShot", with = "bool_as_u8")]
  pub trap_one_shot: Option<NwValue<bool>>,

  /// Represents the trap type of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapType")]
  pub trap_type: Option<NwValue<u8>>,

  /// Represents the type of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Type")]
  pub _type: Option<NwValue<u8>>,

  /// Represents whether the object is usable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Useable", with = "bool_as_u8")]
  pub usable: Option<NwValue<bool>>,

  /// Represents the will saving throw of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Will")]
  pub will: Option<NwValue<u8>>,

  /// Represents the X position of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "X")]
  pub x: Option<NwValue<Decimal>>,

  /// Represents the Y position of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Y")]
  pub y: Option<NwValue<Decimal>>,

  /// Represents the Z position of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Z")]
  pub z: Option<NwValue<Decimal>>,
}

/// TODO
/// Sound
#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {}

/// TODO
/// Store
#[derive(Debug, Serialize, Deserialize)]
pub struct Store {}

/// Trigger
#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
  /// The unique identifier for the struct
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// Indicates whether the key should be automatically removed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "AutoRemoveKey", with = "bool_as_u8")]
  pub auto_remove_key: Option<NwValue<bool>>,

  /// The cursor value associated with the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Cursor")]
  pub cursor: Option<NwValue<u8>>,

  /// The disarm difficulty class (DC) of the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "DisarmDC")]
  pub disarm_dc: Option<NwValue<u8>>,

  /// The faction associated with the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Faction")]
  pub faction: Option<NwValue<u32>>,

  /// The geometry information of the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Geometry")]
  pub geometry: Option<NwValue<Vec<TriggerGeometry>>>,

  /// The height at which the trigger should be highlighted
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HighlightHeight")]
  pub highlight_height: Option<NwValue<Decimal>>,

  /// The name of the key required to interact with the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "KeyName")]
  pub key_name: Option<NwValue<String>>,

  /// The tag of the trigger to which it is linked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
  pub linked_to: Option<NwValue<String>>,

  /// The flags associated with the trigger's linked object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LinkedToFlags")]
  pub linked_to_flags: Option<NwValue<u8>>,

  /// The load screen ID associated with the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LoadScreenID")]
  pub load_screen_id: Option<NwValue<u16>>,

  /// The localized name of the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
  pub localized_name: Option<NwValue<LocalizedText>>,

  /// The script to execute when the trigger is clicked
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnClick")]
  pub on_click: Option<NwValue<String>>,

  /// The script to execute when the trigger is disarmed
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnDisarm")]
  pub on_disarm: Option<NwValue<String>>,

  /// The script to execute when the trigger is triggered as a trap
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "OnTrapTriggered")]
  pub on_trap_triggered: Option<NwValue<String>>,

  /// The portrait ID associated with the trigger
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PortraitId")]
  pub portrait_id: Option<NwValue<u16>>,

  /// The script to execute during the trigger's heartbeat
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ScriptHeartbeat")]
  pub script_heartbeat: Option<NwValue<String>>,

  /// Script to execute when entering the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ScriptOnEnter")]
  pub script_on_enter: Option<NwValue<String>>,

  /// Script to execute when exiting the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ScriptOnExit")]
  pub script_on_exit: Option<NwValue<String>>,

  /// User-defined script
  #[serde(
    default, skip_serializing_if = "Option::is_none",
    rename = "ScriptUserDefine"
  )]
  pub script_user_define: Option<NwValue<String>>,

  /// Tag of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// Template resource reference
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,

  /// Flag indicating whether the trap is detectable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectable", with = "bool_as_u8")]
  pub trap_detectable: Option<NwValue<bool>>,

  /// Trap detection DC (Difficulty Class)
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDetectDC")]
  pub trap_detect_dc: Option<NwValue<u8>>,

  /// Flag indicating whether the trap is disarmable
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapDisarmable", with = "bool_as_u8")]
  pub trap_disarmable: Option<NwValue<bool>>,

  /// Trap flag
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapFlag")]
  pub trap_flag: Option<NwValue<u8>>,

  /// Flag indicating whether the trap is one-shot
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapOneShot", with = "bool_as_u8")]
  pub trap_one_shot: Option<NwValue<bool>>,

  /// Type of trap
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TrapType")]
  pub trap_type: Option<NwValue<u8>>,

  /// Type of the object
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Type")]
  pub _type: Option<NwValue<u8>>,

  /// Orientation on the X-axis
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "XOrientation")]
  pub x_orientation: Option<NwValue<Decimal>>,

  /// Position on the X-axis
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "XPosition")]
  pub x_position: Option<NwValue<Decimal>>,

  /// Orientation on the Y-axis
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "YOrientation")]
  pub y_orientation: Option<NwValue<Decimal>>,

  /// Position on the Y-axis
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "YPosition")]
  pub y_position: Option<NwValue<Decimal>>,

  /// Orientation on the Z-axis
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ZOrientation")]
  pub z_orientation: Option<NwValue<Decimal>>,

  /// Position on the Z-axis
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ZPosition")]
  pub z_position: Option<NwValue<Decimal>>,
}

/// Trigger geometry
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerGeometry {
  /// Identifier for the structure (serialized as "__struct_id")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// X-coordinate of the trigger geometry point (serialized as "PointX")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PointX")]
  pub x: Option<NwValue<Decimal>>,

  /// Y-coordinate of the trigger geometry point (serialized as "PointY")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PointY")]
  pub y: Option<NwValue<Decimal>>,

  /// Z-coordinate of the trigger geometry point (serialized as "PointZ")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "PointZ")]
  pub z: Option<NwValue<Decimal>>,
}

/// Spawn point
#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnPoint {
  /// Identifier for the structure (serialized as "__struct_id")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "__struct_id")]
  pub struct_id: Option<u32>,

  /// Orientation of the spawn point (serialized as "Orientation")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Orientation")]
  pub orientation: Option<NwValue<Decimal>>,

  /// X-coordinate of the spawn point (serialized as "X")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "X")]
  pub x: Option<NwValue<Decimal>>,

  /// Y-coordinate of the spawn point (serialized as "Y")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Y")]
  pub y: Option<NwValue<Decimal>>,

  /// Z-coordinate of the spawn point (serialized as "Z")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Z")]
  pub z: Option<NwValue<Decimal>>,
}

/// Waypoint
#[derive(Debug, Serialize, Deserialize)]
pub struct Waypoint {
  /// Identifier for the structure
  #[serde(rename = "__struct_id")]
  pub struct_id: u32,

  /// Appearance field (serialized as "Appearance")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Appearance")]
  pub appearance: Option<NwValue<i32>>,

  /// Description field (serialized as "Description")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Description")]
  pub description: Option<NwValue<LocalizedText>>,

  /// HasMapNote field (serialized as "HasMapNote")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "HasMapNote", with = "bool_as_u8")]
  pub has_map_note: Option<NwValue<bool>>,

  /// LinkedTo field (serialized as "LinkedTo")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LinkedTo")]
  pub linked_to: Option<NwValue<String>>,

  /// LocalizedName field (serialized as "LocalizedName")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "LocalizedName")]
  pub localized_name: Option<NwValue<LocalizedText>>,

  /// MapNote field (serialized as "MapNote")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MapNote")]
  pub map_note: Option<NwValue<LocalizedText>>,

  /// MapNoteEnabled field (serialized as "MapNoteEnabled")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "MapNoteEnabled", with = "bool_as_u8")]
  pub map_note_enabled: Option<NwValue<bool>>,

  /// Tag field (serialized as "Tag")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tag")]
  pub tag: Option<NwValue<String>>,

  /// TemplateResRef field (serialized as "TemplateResRef")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "TemplateResRef")]
  pub template_res_ref: Option<NwValue<String>>,

  /// XOrientation field (serialized as "XOrientation")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "XOrientation")]
  pub x_orientation: Option<NwValue<Decimal>>,

  /// XPosition field (serialized as "XPosition")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "XPosition")]
  pub x_position: Option<NwValue<Decimal>>,

  /// YOrientation field (serialized as "YOrientation")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "YOrientation")]
  pub y_orientation: Option<NwValue<Decimal>>,

  /// YPosition field (serialized as "YPosition")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "YPosition")]
  pub y_position: Option<NwValue<Decimal>>,

  /// ZPosition field (serialized as "ZPosition")
  #[serde(default, skip_serializing_if = "Option::is_none", rename = "ZPosition")]
  pub z_position: Option<NwValue<Decimal>>,
}
