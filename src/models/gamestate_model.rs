// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Gamestate;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Gamestate = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Number {
    Int(i64),
    Float(f32),
    LongFloat(f64),
    USize(usize),
}

impl Number {
    pub fn to_f64(&self) -> f64 {
        match self {
            Number::Float(f) => *f as f64,
            Number::LongFloat(lf) => *lf,
            Number::Int(i) => *i as f64,
            Number::USize(u) => *u as f64,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Number::Int(i) => *i,
            Number::Float(f) => f.round() as i64,
            Number::LongFloat(lf) => lf.round() as i64,
            Number::USize(u) => *u as i64,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VecOrMap<T> {
    Map(HashMap<String, T>),
    Vec(Vec<T>),
    EmptyVec(Vec<None>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VecOrStruct<T> {
    Struct(T),
    Vec(Vec<T>),
    Empty(Vec<None>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gamestate {
    pub slave_market_manager: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub country: Box<Option<VecOrMap<CountryValue>>>,
    pub leaders: Box<Option<VecOrMap<LeaderValue>>>,
    pub espionage_assets: Option<VecOrMap<EspionageAsset>>,
    pub flags: Box<Option<serde_json::Value>>,
    pub war: Box<Option<VecOrMap<War>>>,
    pub orbital_line: Box<Option<VecOrMap<OrbitalLine>>>,
    pub name: Box<Option<String>>,
    pub random_name_database: Box<Option<RandomNameDatabase>>,
    pub version: Box<Option<String>>,
    pub saved_event_target: Box<Option<Vec<SavedEventTargetElement>>>,
    pub used_species_names: Box<Option<Vec<UsedSpecies>>>,
    pub missile: Box<Option<VecOrMap<MissileValue>>>,
    pub required_dlcs: Box<Option<Vec<String>>>,
    pub natural_wormholes: Box<Option<VecOrMap<NaturalWormhole>>>,
    pub resolution: Box<Option<VecOrMap<Resolution>>>,
    pub planets: Box<Option<Planets>>,
    pub fleet: Box<Option<VecOrMap<FleetValue>>>,
    pub army: Box<Option<VecOrMap<ArmyValue>>>,
    pub last_created_species_ref: Box<Option<Number>>,
    pub buildings: Box<Option<VecOrMap<BuildingValue>>>,
    pub used_symbols: Box<Option<Vec<Number>>>,
    pub last_created_country: Box<Option<Number>>,
    pub galaxy: Box<Option<Galaxy>>,
    pub used_species_portrait: Box<Option<Vec<UsedSpecies>>>,
    pub galactic_community: Box<Option<GalacticCommunity>>,
    pub strike_craft: Box<Option<VecOrMap<StrikeCraftValue>>>,
    pub last_refugee_country: Box<Option<Number>>,
    pub fired_event_ids: Box<Option<Vec<String>>>,
    pub player: Box<Option<Vec<Player>>>,
    pub truce: Box<Option<VecOrMap<TruceValue>>>,
    pub last_event_id: Box<Option<Number>>,
    pub agreements: Box<Option<Agreements>>,
    pub last_created_army: Box<Option<Number>>,
    pub last_created_leader: Box<Option<Number>>,
    pub spy_networks: Box<Option<VecOrMap<SpyNetwork>>>,
    pub saved_leaders: Box<Option<VecOrStruct<SavedLeaders>>>,
    pub galactic_object: Box<Option<VecOrMap<GalacticObject>>>,
    pub name_list: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub last_created_system: Box<Option<Number>>,
    pub construction: Box<Option<Construction>>,
    pub galaxy_radius: Box<Option<Number>>,
    pub debris: Box<Option<DebrisOrValue>>,
    pub trade_deal: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub sectors: Box<Option<VecOrMap<Sector>>>,
    pub espionage_operations: Box<Option<EspionageOperations>>,
    pub first_contacts: Box<Option<FirstContacts>>,
    pub additional_crisis_strength: Box<Option<Number>>,
    pub last_created_pop: Box<Option<Number>>,
    pub date: Box<Option<String>>,
    pub archaeological_sites: Box<Option<ArchaeologicalSites>>,
    pub random_log_day: Box<Option<Number>>,
    pub tick: Box<Option<Number>>,
    pub situations: Box<Option<Situations>>,
    pub deposit: Box<Option<VecOrMap<DepositValue>>>,
    pub nebula: Box<Option<Vec<Nebula>>>,
    pub random_seed: Box<Option<Number>>,
    pub megastructures: Box<Option<HashMap<String, Megastructure>>>,
    pub pop: Box<Option<VecOrMap<Pop>>>,
    pub last_notification_id: Box<Option<Number>>,
    pub bypasses: Box<Option<VecOrMap<Bypass>>>,
    pub trade_routes_manager: Box<Option<TradeRoutesManager>>,
    pub last_created_fleet: Box<Option<Number>>,
    pub dummy_species: Box<Option<Number>>,
    pub global_ship_design: Box<Option<Vec<GlobalShipDesign>>>,
    pub ship_design: Box<Option<VecOrMap<ShipDesignValue>>>,
    pub last_killed_country_name: Box<Option<LastKilledCountryName>>,
    pub version_control_revision: Box<Option<Number>>,
    pub open_player_event_selection_history: Box<Option<OpenPlayerEventSelectionHistory>>,
    pub used_color: Box<Option<Vec<String>>>,
    pub starbase_mgr: Box<Option<StarbaseMgr>>,
    pub random_count: Box<Option<Number>>,
    pub rim_galactic_objects: Box<Option<Vec<Number>>>,
    pub clusters: Box<Option<Vec<Cluster>>>,
    pub last_created_design: Box<Option<Number>>,
    pub last_diplo_action_id: Box<Option<Number>>,
    pub species_db: Box<Option<VecOrMap<SpeciesDb>>>,
    pub message: Box<Option<Vec<Message>>>,
    pub fleet_template: Box<Option<VecOrMap<FleetTemplateValue>>>,
    pub ships: Box<Option<VecOrMap<Ship>>>,
    pub last_created_ship: Box<Option<Number>>,
    pub federation: Box<Option<Federation>>,
    pub ground_combat: Box<Option<GroundCombat>>,
    pub trade_routes: Box<Option<VecOrMap<TradeRoute>>>,
    pub system_initializer_counter: Box<Option<SystemInitializerCounter>>,
    pub ambient_object: Box<Option<serde_json::Value>>,
    pub market: Box<Option<Market>>,
    pub last_created_ambient_object: Box<Option<Number>>,
    pub council_positions: Box<Option<CouncilPositions>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EspionageAsset {
    #[serde(rename = "type")]
    pub espionage_asset_type: Option<String>,
    pub name: Option<EspionageAssetName>,
    pub species_ref: Option<Number>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EspionageAssetName {
    pub use_full_regnal_name: Option<String>,
    pub full_names: Option<FullNames>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullNames {
    pub key: Option<String>,
    pub variables: Option<Vec<FullNamesVariable>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullNamesVariable {
    pub key: Option<String>,
    pub value: Option<LastKilledCountryName>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TruceValue {
    String(String),
    TruceClass(TruceClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TruceClass {
    pub name: Option<LastKilledCountryName>,
    pub start_date: Option<String>,
    pub truce_type: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agreements {
    pub agreements: Box<Option<VecOrMap<Box<serde_json::Value>>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AmbientObjectValue {
    AmbientObjectClass(AmbientObjectClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmbientObjectClass {
    pub flags: Box<Option<VecOrMap<Number>>>,
    pub data: Box<Option<String>>,
    pub properties: Box<Option<Properties>>,
    pub coordinate: Box<Option<Position>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub y: Box<Option<Number>>,
    pub origin: Box<Option<Number>>,
    pub x: Box<Option<Number>>,
    pub visual_height: Box<Option<Number>>,
    pub randomized: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    pub coordinate: Box<Option<Position>>,
    pub offset: Box<Option<Vec<Number>>>,
    pub attach: Box<Option<Owner>>,
    pub scale: Box<Option<Number>>,
    pub entity_face_object: Box<Option<Owner>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Owner {
    pub id: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub owner_type: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchaeologicalSites {
    pub sites: Box<Option<VecOrMap<Site>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    pub index: Box<Option<Number>>,
    pub last_roll: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub site_type: Box<Option<String>>,
    pub clues: Box<Option<Number>>,
    pub location: Box<Option<Owner>>,
    pub excavator_fleet: Box<Option<Number>>,
    pub last_excavator_country: Box<Option<Number>>,
    pub days_left: Box<Option<Number>>,
    pub difficulty: Box<Option<Number>>,
    pub locked: Box<Option<String>>,
    pub visible_to: Box<Option<Vec<Number>>>,
    pub log: Box<Option<Vec<Log>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub bonus: Box<Option<Number>>,
    pub difficulty: Box<Option<Number>>,
    pub tooltip: Box<Option<String>>,
    pub date: Box<Option<String>>,
    pub clues: Box<Option<Number>>,
    pub title: Box<Option<String>>,
    pub total: Box<Option<Number>>,
    pub roll: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ArmyValue {
    ArmyClass(ArmyClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArmyClass {
    pub max_health: Box<Option<Number>>,
    pub health: Box<Option<Number>>,
    pub morale: Box<Option<Number>>,
    pub home_planet: Box<Option<Number>>,
    pub ship: Box<Option<Number>>,
    pub owner: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub army_type: Box<Option<String>>,
    pub name: Box<Option<ArmyName>>,
    pub leader: Box<Option<Number>>,
    pub jump_drive_cooldown: Box<Option<String>>,
    pub species: Box<Option<Number>>,
    pub planet: Box<Option<Number>>,
    pub pop: Box<Option<Number>>,
    pub army_modifier: Box<Option<ArmyModifier>>,
    pub experience: Box<Option<Number>>,
    pub is_defense_army_mod_spawned: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArmyModifier {
    pub army_defense_health_mult: Box<Option<Number>>,
    pub army_defense_damage_mult: Box<Option<Number>>,
    pub army_defense_morale_mult: Box<Option<Number>>,
    pub army_attack_damage_mult: Box<Option<Number>>,
    pub army_attack_health_mult: Box<Option<Number>>,
    pub army_starting_experience_add: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArmyName {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<PurpleVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleVariable {
    pub value: Box<Option<PurpleValue>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleValue {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<FluffyVariable>>>,
    pub literal: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffyVariable {
    pub key: Box<Option<String>>,
    pub value: Box<Option<SectorName>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SectorName {
    pub key: Box<Option<String>>,
    pub literal: Box<Option<String>>,
    pub variables: Box<Option<Vec<TentacledVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TentacledVariable {
    pub key: Box<Option<String>>,
    pub value: Box<Option<PluralClass>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluralClass {
    pub key: Box<Option<String>>,
    pub literal: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BuildingValue {
    BuildingClass(BuildingClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildingClass {
    pub position: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub building_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bypass {
    pub owner: Box<Option<Owner>>,
    pub connections: Box<Option<Vec<Number>>>,
    #[serde(rename = "type")]
    pub bypass_type: Box<Option<String>>,
    pub active_connections: Box<Option<Vec<Number>>>,
    pub active: Box<Option<String>>,
    pub linked_to: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cluster {
    pub id: Box<Option<String>>,
    pub radius: Box<Option<Number>>,
    pub position: Box<Option<Position>>,
    pub objects: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Construction {
    pub queue_mgr: Box<Option<QueueMgr>>,
    pub item_mgr: Box<Option<ItemMgr>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemMgr {
    pub items: Box<Option<VecOrMap<ItemValue>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ItemValue {
    ItemItem(ItemItem),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemItem {
    pub buildable_district: Box<Option<BuildableDistrict>>,
    pub paying_country: Box<Option<Number>>,
    pub progress: Box<Option<Number>>,
    pub resources: Box<Option<ItemResources>>,
    pub progress_needed: Box<Option<Number>>,
    pub queue: Box<Option<Number>>,
    pub buildable_planet_building: Box<Option<BuildablePlanetBuilding>>,
    pub buildable_replace_district: Box<Option<BuildableReplaceDistrict>>,
    pub buildable_ship: Box<Option<BuildableShip>>,
    pub buildable_starbase_building: Box<Option<BuildableStarbaseBuilding>>,
    pub buildable_starbase_upgrade: Box<Option<BuildableStarbaseUpgrade>>,
    pub buildable_starbase_module: Box<Option<BuildableStarbaseModule>>,
    pub buildable_federation_ship: Box<Option<BuildableShip>>,
    pub buildable_decision: Box<Option<BuildableDecision>>,
    pub buildable_colony_ship: Box<Option<BuildableColonyShip>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableColonyShip {
    pub ship_design: Box<Option<Number>>,
    pub species: Box<Option<Number>>,
    pub orbitable: Box<Option<BuildableColonyShipOrbitable>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableColonyShipOrbitable {
    pub starbase: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableDecision {
    pub planet: Box<Option<Number>>,
    pub decision: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableDistrict {
    pub district: Box<Option<String>>,
    pub planet: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableShip {
    pub ship_design: Box<Option<Number>>,
    pub orbitable: Box<Option<BuildableColonyShipOrbitable>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildablePlanetBuilding {
    pub planet: Box<Option<Number>>,
    pub building: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableReplaceDistrict {
    pub district: Box<Option<String>>,
    pub planet: Box<Option<Number>>,
    pub replace: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableStarbaseBuilding {
    pub starbase_building: Box<Option<String>>,
    pub starbase: Box<Option<Number>>,
    pub slot: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableStarbaseModule {
    pub starbase_module: Box<Option<String>>,
    pub slot: Box<Option<Number>>,
    pub starbase: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildableStarbaseUpgrade {
    pub starbase_upgrade: Box<Option<String>>,
    pub starbase: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemResources {
    pub minerals: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueueMgr {
    pub queues: Box<Option<VecOrMap<Queue>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Queue {
    #[serde(rename = "type")]
    pub queue_type: Box<Option<String>>,
    pub location: Box<Option<Owner>>,
    pub simultaneous: Box<Option<Number>>,
    pub disabled: Box<Option<String>>,
    pub owner: Box<Option<Number>>,
    pub items: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouncilPositions {
    pub council_positions: Box<Option<VecOrMap<CouncilPositionValue>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CouncilPositionValue {
    CouncilPositionClass(CouncilPositionClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouncilPositionClass {
    #[serde(rename = "type")]
    pub council_position_type: Box<Option<String>>,
    pub leader: Box<Option<Number>>,
    pub country: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CountryValue {
    CountryClass(CountryClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryClass {
    pub flag: Box<Option<Flag>>,
    pub color_index: Box<Option<Number>>,
    pub name: Box<Option<PurpleName>>,
    pub adjective: Box<Option<CountryAdjective>>,
    pub custom_name: Box<Option<String>>,
    pub tech_status: Box<Option<TechStatus>>,
    pub last_date_was_human: Box<Option<String>>,
    pub last_date_war_lost: Box<Option<String>>,
    pub last_date_at_war: Box<Option<String>>,
    pub budget: Box<Option<Budget>>,
    pub events: Box<Option<EventsUnion>>,
    pub track_all_situations: Box<Option<String>>,
    pub terra_incognita: Box<Option<TerraIncognita>>,
    pub military_power: Box<Option<Number>>,
    pub economy_power: Box<Option<Number>>,
    pub victory_rank: Box<Option<Number>>,
    pub victory_score: Box<Option<Number>>,
    pub tech_power: Box<Option<Number>>,
    pub immigration: Box<Option<Number>>,
    pub emigration: Box<Option<Number>>,
    pub fleet_size: Box<Option<Number>>,
    pub used_naval_capacity: Box<Option<Number>>,
    pub empire_size: Box<Option<Number>>,
    pub new_colonies: Box<Option<Number>>,
    pub sapient: Box<Option<Number>>,
    pub graphical_culture: Box<Option<String>>,
    pub city_graphical_culture: Box<Option<String>>,
    pub room: Box<Option<String>>,
    pub ai: Box<Option<Ai>>,
    pub capital: Box<Option<Number>>,
    pub founder_species_ref: Box<Option<Number>>,
    pub ethos: Box<Option<CountryEthos>>,
    pub last_alliance_name: Box<Option<LastAllianceName>>,
    pub fleet_template_manager: Box<Option<FleetTemplateManagerUnion>>,
    pub government: Box<Option<Government>>,
    pub advisor_voice_type: Box<Option<String>>,
    pub personality: Box<Option<String>>,
    pub next_election: Box<Option<String>>,
    pub government_date: Box<Option<String>>,
    pub surveyed: Box<Option<Vec<Number>>>,
    pub homeworld_name: Box<Option<PluralClass>>,
    pub visited_objects: Box<Option<Vec<Number>>>,
    pub intel_level: Box<Option<Vec<Number>>>,
    pub highest_intel_level: Box<Option<Vec<Number>>>,
    pub intel: Box<Option<Vec<CountryIntel>>>,
    pub detection_intel: Box<Option<Vec<DetectionIntel>>>,
    pub default_planet_automation_settings: Box<Option<Vec<String>>>,
    pub timed_modifier: Box<Option<CountryTimedModifier>>,
    pub flags: Box<Option<CountryFlags>>,
    pub next_transport_fleet_number: Box<Option<Number>>,
    pub next_army_number: Box<Option<Number>>,
    pub sensor_range_fleets: Box<Option<Vec<Number>>>,
    pub space_critter: Box<Option<Vec<Number>>>,
    pub faction: Box<Option<Faction>>,
    pub name_list: Box<Option<String>>,
    pub ship_names: Box<Option<HashMap<String, Number>>>,
    pub ruler: Box<Option<Number>>,
    pub control_groups: Box<Option<VecOrMap<Vec<Owner>>>>,
    pub ship_prefix: Box<Option<LastAllianceName>>,
    pub active_policies: Box<Option<Vec<ActivePolicy>>>,
    pub policy_flags: Box<Option<Vec<String>>>,
    pub federation: Box<Option<Number>>,
    pub subjects: Box<Option<Vec<Number>>>,
    pub starting_system: Box<Option<Number>>,
    pub has_advisor: Box<Option<String>>,
    pub shown_message_types: Box<Option<Vec<String>>>,
    pub owned_leaders: Box<Option<Vec<Number>>>,
    pub owned_megastructures: Box<Option<Vec<Number>>>,
    pub tradition_categories: Box<Option<Vec<String>>>,
    pub traditions: Box<Option<Vec<String>>>,
    pub ascension_perks: Box<Option<Vec<String>>>,
    pub owned_armies: Box<Option<Vec<Number>>>,
    pub owned_planets: Box<Option<Vec<Number>>>,
    pub holding_planets: Box<Option<Vec<HoldingPlanet>>>,
    pub restricted_systems: Box<Option<Vec<Number>>>,
    pub controlled_planets: Box<Option<Vec<Number>>>,
    pub ship_design_collection: Box<Option<CountryShipDesignCollection>>,
    pub edicts: Box<Option<Vec<Edict>>>,
    #[serde(rename = "type")]
    pub country_type: Box<Option<String>>,
    pub modules: Box<Option<Modules>>,
    pub initialized: Box<Option<String>>,
    pub relations_manager: Box<Option<RelationsManager>>,
    pub espionage_manager: Box<Option<EspionageManagerUnion>>,
    pub intel_manager: Box<Option<IntelManager>>,
    pub fleets_manager: Box<Option<FleetsManagerUnion>>,
    pub war_allies: Box<Option<Vec<Number>>>,
    pub customization: Box<Option<String>>,
    pub is_in_breach_of_any: Box<Option<String>>,
    pub awareness: Box<Option<Number>>,
    pub old_awareness: Box<Option<Number>>,
    pub last_changed_country_type: Box<Option<String>>,
    pub seen_bypass_types: Box<Option<Vec<String>>>,
    pub relics: Box<Option<Vec<String>>>,
    pub last_received_relic: Box<Option<String>>,
    pub usable_bypasses: Box<Option<Vec<Number>>>,
    pub hyperlane_systems: Box<Option<Vec<Number>>>,
    pub sectors: Box<Option<Sectors>>,
    pub given_value: Box<Option<f64>>,
    pub num_upgraded_starbase: Box<Option<Number>>,
    pub starbase_capacity: Box<Option<Number>>,
    pub employable_pops: Box<Option<Number>>,
    pub owned_species_refs: Box<Option<Vec<Number>>>,
    pub first_contact: Box<Option<FirstContactUnion>>,
    pub built_species_ref: Box<Option<Number>>,
    pub seen_bypasses: Box<Option<Vec<Number>>>,
    pub last_activated_relic: Box<Option<String>>,
    pub enslaved_species_refs: Box<Option<Vec<Number>>>,
    pub overlord: Box<Option<Number>>,
    pub subject_date: Box<Option<String>>,
    pub loyalty: Box<Option<f64>>,
    pub regnal_numbers: Box<Option<Vec<RegnalNumber>>>,
    pub last_lost_relic: Box<Option<String>>,
    pub associated_federation: Box<Option<Number>>,
    pub incoming_actions: Box<Option<Vec<IncomingAction>>>,
    pub preftl_age: Box<Option<String>>,
    pub location: Box<Option<Owner>>,
    pub rebelling_against: Box<Option<Number>>,
    pub rebel: Box<Option<String>>,
    pub is_affected_by_war_exhaustion: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectionIntel {
    pub star: Option<Number>,
    pub intel: Option<Vec<DetectionIntelIntel>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectionIntelIntel {
    pub strength: Option<Number>,
    pub timestamp: Option<String>,
    pub country: Option<Number>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivePolicy {
    pub policy: Box<Option<String>>,
    pub selected: Box<Option<String>>,
    pub date: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryAdjective {
    pub variables: Box<Option<Vec<TentacledVariable>>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ai {
    pub building: Box<Option<String>>,
    pub prepare_war: Box<Option<String>>,
    pub can_build_robots: Box<Option<String>>,
    pub random_count: Box<Option<Number>>,
    pub prepare_war_date_start: Box<Option<String>>,
    pub war: Box<Option<String>>,
    pub ai_death_spiral: Box<Option<String>>,
    pub robot_colonies_with_free_buildings: Box<Option<Number>>,
    pub prepare_war_date: Box<Option<String>>,
    pub ai_resource_target_records: Box<Option<AiResourceTargetRecords>>,
    pub navy_depleted: Box<Option<String>>,
    pub synced_random_seed: Box<Option<Number>>,
    pub last_diplo_actions: Box<Option<Vec<LastDiploAction>>>,
    pub station: Box<Option<String>>,
    pub attitude: Box<Option<Vec<Attitude>>>,
    pub budget: Box<Option<Vec<Number>>>,
    pub deficit_spending: Box<Option<String>>,
    pub strategy: Box<Option<Vec<Strategy>>>,
    pub robot_colonies: Box<Option<Number>>,
    pub random_seed: Box<Option<Number>>,
    pub synced_random_count: Box<Option<Number>>,
    pub need_strategic_war_data: Box<Option<String>>,
    pub colonize: Box<Option<String>>,
    pub initialized: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AiResourceTargetRecords {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    DoubleMap(VecOrMap<Number>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attitude {
    pub priority: Box<Option<Number>>,
    pub date: Box<Option<String>>,
    pub weight: Box<Option<Number>>,
    pub attitude: Box<Option<String>>,
    pub country: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastDiploAction {
    pub country: Box<Option<Number>>,
    pub action: Box<Option<String>>,
    pub creation_date: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Strategy {
    pub target: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub strategy_type: Box<Option<Number>>,
    pub id: Box<Option<Number>>,
    pub value: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Budget {
    pub current_month: Box<Option<TMonth>>,
    pub last_month: Box<Option<TMonth>>,
    pub income_high_water_mark: Box<Option<IncomeHighWaterMark>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TMonth {
    pub trade_balance: Box<Option<ExtraBalance>>,
    pub trade_income: Box<Option<ExtraBalance>>,
    pub expenses: Box<Option<Expenses>>,
    pub extra_income: Box<Option<ExtraBalance>>,
    pub extra_balance: Box<Option<ExtraBalance>>,
    pub extra_expenses: Box<Option<ExtraBalance>>,
    pub trade_expenses: Box<Option<ExtraBalance>>,
    pub income: Box<Option<Income>>,
    pub balance: Box<Option<Balance>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    pub station_gatherers: Box<Option<Campaigns>>,
    pub sbtg_light_reactor: Box<Option<Campaigns>>,
    pub orbital_mining_deposits: Box<Option<OrbitalMiningDeposits>>,
    pub colonies: Box<Option<Campaigns>>,
    pub planet_districts: Box<Option<VecOrMap<Number>>>,
    pub leader_admirals: Box<Option<DiplomaticNetworking>>,
    pub country_ruler: Box<Option<CommercialPacts>>,
    pub country_tech: Box<Option<CommercialPacts>>,
    pub station_researchers: Box<Option<Campaigns>>,
    pub pop_category_precursor: Box<Option<PopCategoryPrecursor>>,
    pub planet_administrators: Box<Option<BalancePlanetAdministrators>>,
    pub starbase_modules: Box<Option<Campaigns>>,
    pub planet_buildings: Box<Option<VecOrMap<Number>>>,
    pub country_traditions: Box<Option<CommercialPacts>>,
    pub country_power_projection: Box<Option<CommercialPacts>>,
    pub leader_scientists: Box<Option<LeaderScientists>>,
    pub megastructures: Box<Option<Megastructures>>,
    pub orbital_research_deposits: Box<Option<OrbitalResearchDeposits>>,
    pub country_base: Box<Option<VecOrMap<Number>>>,
    pub planet_jobs: Box<Option<BalancePlanetJobs>>,
    pub armies: Box<Option<LeaderScientistsClass>>,
    pub leader_governors: Box<Option<DiplomaticNetworking>>,
    pub none: Box<Option<None>>,
    pub planet_miners: Box<Option<BalancePlanetMiners>>,
    pub rivalries: Box<Option<CommercialPacts>>,
    pub planet_technician: Box<Option<PlanetTechnician>>,
    pub ships: Box<Option<Ships>>,
    pub planet_farmers: Box<Option<BalancePlanetFarmers>>,
    pub leader_generals: Box<Option<BalanceLeaderGenerals>>,
    pub trade_routes: Box<Option<TradeRoutes>>,
    pub planet_districts_mining: Box<Option<Campaigns>>,
    pub planet_managers: Box<Option<PlanetEntertainersClass>>,
    pub planet_metallurgists: Box<Option<PlanetMetallurgists>>,
    pub planet_branch_offices: Box<Option<Campaigns>>,
    pub planet_districts_cities: Box<Option<Campaigns>>,
    pub planet_districts_farming: Box<Option<Campaigns>>,
    pub starbase_buildings: Box<Option<BalanceStarbaseBuildings>>,
    pub country_ethic: Box<Option<CommercialPacts>>,
    pub research_agreements: Box<Option<CommercialPacts>>,
    pub pop_category_workers: Box<Option<BalancePopCategoryRulers>>,
    pub planet_executives: Box<Option<PlanetEntertainersClass>>,
    pub planet_districts_industrial: Box<Option<Campaigns>>,
    pub planet_artisans: Box<Option<BalancePlanetArtisans>>,
    pub commercial_pacts: Box<Option<CommercialPacts>>,
    pub migration_pacts: Box<Option<CommercialPacts>>,
    pub starbase_stations: Box<Option<Campaigns>>,
    pub planet_districts_generator: Box<Option<Campaigns>>,
    pub pop_category_specialists: Box<Option<BalancePopCategoryRulers>>,
    pub pop_category_rulers: Box<Option<BalancePopCategoryRulers>>,
    pub campaigns: Box<Option<Campaigns>>,
    pub planet_researchers: Box<Option<GigaGigastructuresClass>>,
    pub planet_branch_office_buildings: Box<Option<Campaigns>>,
    pub ship_components: Box<Option<ShipComponents>>,
    pub non_aggression_pacts: Box<Option<CommercialPacts>>,
    pub pop_category_robot: Box<Option<Campaigns>>,
    pub planet_politicians: Box<Option<PlanetEntertainersClass>>,
    pub planet_pop_assemblers: Box<Option<PlanetPopAssemblersClass>>,
    pub station_observers: Box<Option<Campaigns>>,
    pub station_observer_missions: Box<Option<PlanetCultureWorkers>>,
    pub planet_evaluators: Box<Option<PlanetEvaluatorsClass>>,
    pub pop_category_drones: Box<Option<PopCategoryDrones>>,
    pub unemployment_resources: Box<Option<UnemploymentResources>>,
    pub defensive_pacts: Box<Option<CommercialPacts>>,
    pub planet_priests: Box<Option<PlanetEntertainersClass>>,
    pub federation: Box<Option<CommercialPacts>>,
    pub envoy_costs: Box<Option<CommercialPacts>>,
    pub planet_merchants: Box<Option<PlanetMerchants>>,
    pub pop_category_slave: Box<Option<PopCategorySlave>>,
    pub edicts: Box<Option<DiplomaticNetworking>>,
    pub planet_nobles: Box<Option<PlanetEntertainersClass>>,
    pub planet_entertainers: Box<Option<PlanetEntertainersClass>>,
    pub diplomatic_networking: Box<Option<DiplomaticNetworking>>,
    pub planet_culture_workers: Box<Option<PlanetCultureWorkers>>,
    pub planet_necro_apprentices: Box<Option<PlanetNecroApprentices>>,
    pub planet_bio_trophies: Box<Option<DiplomaticNetworking>>,
    pub pop_category_bio_trophy: Box<Option<PopCategoryBioTrophy>>,
    pub giga_megastructures: Box<Option<VecOrMap<Number>>>,
    pub country_civics: Box<Option<CommercialPacts>>,
    pub pop_category_xeno_ward: Box<Option<PopCategoryXenoWardElement>>,
    pub giga_kilostructures: Box<Option<GigaStructures>>,
    pub giga_gigastructures: Box<Option<BalanceGigaGigastructures>>,
    pub planet_deposits: Box<Option<PlanetDeposits>>,
    pub planet_telepaths: Box<Option<PlanetEvaluatorsClass>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderScientistsClass {
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub influence: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Campaigns {
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommercialPacts {
    pub influence: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiplomaticNetworking {
    pub unity: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceGigaGigastructures {
    pub physics_research: Box<Option<Number>>,
    pub engineering_research: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub influence: Box<Option<Number>>,
    pub society_research: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GigaStructures {
    pub alloys: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub influence: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceLeaderGenerals {
    pub minerals: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderScientists {
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Megastructures {
    pub engineering_research: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub acot_sr_dark_energy: Box<Option<Number>>,
    pub sr_dark_matter: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum None {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    CommercialPacts(CommercialPacts),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitalMiningDeposits {
    pub sr_zro: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub volatile_motes: Box<Option<Number>>,
    pub exotic_gases: Box<Option<Number>>,
    pub rare_crystals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitalResearchDeposits {
    pub physics_research: Box<Option<Number>>,
    pub engineering_research: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub society_research: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minor_artifacts: Box<Option<Number>>,
    pub sr_dark_matter: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalancePlanetAdministrators {
    pub unity: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalancePlanetArtisans {
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetCultureWorkers {
    pub society_research: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetDeposits {
    pub alloys: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetEntertainersClass {
    pub consumer_goods: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetEvaluatorsClass {
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalancePlanetFarmers {
    pub consumer_goods: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalancePlanetJobs {
    pub engineering_research: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub society_research: Box<Option<Number>>,
    pub physics_research: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minor_artifacts: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetMerchants {
    pub consumer_goods: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetMetallurgists {
    pub alloys: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalancePlanetMiners {
    pub alloys: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub acot_sr_stellarite: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetNecroApprentices {
    pub food: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetPopAssemblersClass {
    pub alloys: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GigaGigastructuresClass {
    pub engineering_research: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub society_research: Box<Option<Number>>,
    pub physics_research: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minor_artifacts: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetTechnician {
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategoryBioTrophy {
    pub consumer_goods: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategoryDrones {
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategoryPrecursor {
    pub unity: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalancePopCategoryRulers {
    pub consumer_goods: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub rare_crystals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategorySlave {
    pub food: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategoryXenoWardElement {
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipComponents {
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub ehof_sr_sentient_metal: Box<Option<Number>>,
    pub sr_zro: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ships {
    pub energy: Box<Option<Number>>,
    pub acot_sr_dark_energy: Box<Option<Number>>,
    pub sr_dark_matter: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceStarbaseBuildings {
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub sr_dark_matter: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeRoutes {
    pub energy: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnemploymentResources {
    pub minerals: Box<Option<Number>>,
    pub engineering_research: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub society_research: Box<Option<Number>>,
    pub physics_research: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub minor_artifacts: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expenses {
    pub planet_jobs: Box<Option<ExpensesPlanetJobs>>,
    pub armies: Box<Option<ExpensesArmies>>,
    pub colonies: Box<Option<Campaigns>>,
    pub station_researchers: Box<Option<Campaigns>>,
    pub none: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub sbtg_light_reactor: Box<Option<Campaigns>>,
    pub leader_generals: Box<Option<DiplomaticNetworking>>,
    pub planet_buildings: Box<Option<GigaKilostructures>>,
    pub leader_admirals: Box<Option<DiplomaticNetworking>>,
    pub station_gatherers: Box<Option<Campaigns>>,
    pub leader_scientists: Box<Option<LeaderScientists>>,
    pub leader_governors: Box<Option<DiplomaticNetworking>>,
    pub ships: Box<Option<Ships>>,
    pub ship_components: Box<Option<ShipComponents>>,
    pub planet_districts_industrial: Box<Option<Campaigns>>,
    pub planet_branch_office_buildings: Box<Option<Campaigns>>,
    pub planet_metallurgists: Box<Option<PlanetMetallurgistsClass>>,
    pub planet_districts_cities: Box<Option<Campaigns>>,
    pub planet_researchers: Box<Option<ExpensesPlanetResearchers>>,
    pub planet_executives: Box<Option<PlanetMerchants>>,
    pub planet_districts_mining: Box<Option<Campaigns>>,
    pub planet_artisans: Box<Option<PlanetMetallurgistsClass>>,
    pub pop_category_specialists: Box<Option<PopCategoryPrecursorClass>>,
    pub planet_districts_farming: Box<Option<Campaigns>>,
    pub pop_category_workers: Box<Option<PopCategoryPrecursorClass>>,
    pub starbase_stations: Box<Option<Campaigns>>,
    pub planet_managers: Box<Option<PlanetMerchants>>,
    pub non_aggression_pacts: Box<Option<CommercialPacts>>,
    pub research_agreements: Box<Option<CommercialPacts>>,
    pub starbase_modules: Box<Option<Campaigns>>,
    pub planet_districts_generator: Box<Option<Campaigns>>,
    pub pop_category_rulers: Box<Option<PopCategoryPrecursorClass>>,
    pub campaigns: Box<Option<Campaigns>>,
    pub commercial_pacts: Box<Option<CommercialPacts>>,
    pub migration_pacts: Box<Option<CommercialPacts>>,
    pub starbase_buildings: Box<Option<ExpensesStarbaseBuildings>>,
    pub planet_politicians: Box<Option<PlanetMerchants>>,
    pub planet_administrators: Box<Option<ExpensesPlanetAdministrators>>,
    pub federation: Box<Option<CommercialPacts>>,
    pub envoy_costs: Box<Option<CommercialPacts>>,
    pub defensive_pacts: Box<Option<CommercialPacts>>,
    pub planet_pop_assemblers: Box<Option<PlanetPopAssemblersClass>>,
    pub unemployment_resources: Box<Option<TradeRoutes>>,
    pub planet_merchants: Box<Option<PlanetMerchants>>,
    pub pop_category_slave: Box<Option<PopCategorySlave>>,
    pub planet_priests: Box<Option<PlanetMerchants>>,
    pub station_observers: Box<Option<Campaigns>>,
    pub planet_nobles: Box<Option<PlanetMerchants>>,
    pub planet_necro_apprentices: Box<Option<PopCategoryBioTrophy>>,
    pub planet_technician: Box<Option<Campaigns>>,
    pub edicts: Box<Option<DiplomaticNetworking>>,
    pub pop_category_robot: Box<Option<Campaigns>>,
    pub pop_category_drones: Box<Option<PopCategoryDrones>>,
    pub planet_evaluators: Box<Option<Campaigns>>,
    pub planet_entertainers: Box<Option<PlanetMerchants>>,
    pub pop_category_bio_trophy: Box<Option<PopCategoryBioTrophy>>,
    pub pop_category_precursor: Box<Option<PopCategoryPrecursorClass>>,
    pub country_civics: Box<Option<CommercialPacts>>,
    pub giga_megastructures: Box<Option<GigaStructures>>,
    pub planet_farmers: Box<Option<Campaigns>>,
    pub planet_miners: Box<Option<Campaigns>>,
    pub pop_category_xeno_ward: Box<Option<PopCategoryXenoWardElement>>,
    pub planet_districts: Box<Option<PlanetDistricts>>,
    pub giga_gigastructures: Box<Option<ExpensesGigaGigastructures>>,
    pub giga_kilostructures: Box<Option<GigaKilostructures>>,
    pub country_power_projection: Box<Option<CommercialPacts>>,
    pub country_tech: Box<Option<CommercialPacts>>,
    pub country_ruler: Box<Option<CommercialPacts>>,
    pub planet_telepaths: Box<Option<Campaigns>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesArmies {
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesGigaGigastructures {
    pub alloys: Box<Option<Number>>,
    pub influence: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GigaKilostructures {
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesPlanetAdministrators {
    pub consumer_goods: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetMetallurgistsClass {
    pub minerals: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetDistricts {
    pub rare_crystals: Box<Option<Number>>,
    pub exotic_gases: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub volatile_motes: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesPlanetJobs {
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesPlanetResearchers {
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategoryPrecursorClass {
    pub consumer_goods: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpensesStarbaseBuildings {
    pub energy: Box<Option<Number>>,
    pub sr_dark_matter: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtraBalance {
    pub none: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Income {
    pub planet_buildings: Box<Option<VecOrMap<Number>>>,
    pub orbital_research_deposits: Box<Option<OrbitalResearchDeposits>>,
    pub country_ruler: Box<Option<CommercialPacts>>,
    pub starbase_modules: Box<Option<Campaigns>>,
    pub planet_miners: Box<Option<IncomePlanetMiners>>,
    pub country_tech: Box<Option<CommercialPacts>>,
    pub rivalries: Box<Option<CommercialPacts>>,
    pub country_power_projection: Box<Option<CommercialPacts>>,
    pub megastructures: Box<Option<Megastructures>>,
    pub planet_farmers: Box<Option<IncomePlanetFarmers>>,
    pub orbital_mining_deposits: Box<Option<OrbitalMiningDeposits>>,
    pub leader_generals: Box<Option<IncomeLeaderGenerals>>,
    pub planet_districts: Box<Option<VecOrMap<Number>>>,
    pub country_base: Box<Option<VecOrMap<Number>>>,
    pub planet_technician: Box<Option<PlanetTechnician>>,
    pub pop_category_precursor: Box<Option<DiplomaticNetworking>>,
    pub none: Box<Option<None>>,
    pub planet_jobs: Box<Option<BalancePlanetJobs>>,
    pub planet_administrators: Box<Option<DiplomaticNetworking>>,
    pub country_traditions: Box<Option<CommercialPacts>>,
    pub trade_routes: Box<Option<TradeRoutes>>,
    pub planet_executives: Box<Option<DiplomaticNetworking>>,
    pub starbase_buildings: Box<Option<PlanetMetallurgistsClass>>,
    pub planet_branch_offices: Box<Option<Campaigns>>,
    pub planet_metallurgists: Box<Option<PlanetPopAssemblersClass>>,
    pub country_ethic: Box<Option<CommercialPacts>>,
    pub planet_artisans: Box<Option<PurplePlanetArtisans>>,
    pub planet_researchers: Box<Option<GigaGigastructuresClass>>,
    pub planet_managers: Box<Option<DiplomaticNetworking>>,
    pub planet_politicians: Box<Option<DiplomaticNetworking>>,
    pub diplomatic_networking: Box<Option<DiplomaticNetworking>>,
    pub planet_priests: Box<Option<DiplomaticNetworking>>,
    pub leader_governors: Box<Option<DiplomaticNetworking>>,
    pub planet_nobles: Box<Option<DiplomaticNetworking>>,
    pub station_observer_missions: Box<Option<PlanetCultureWorkers>>,
    pub planet_culture_workers: Box<Option<PlanetCultureWorkers>>,
    pub planet_evaluators: Box<Option<DiplomaticNetworking>>,
    pub unemployment_resources: Box<Option<GigaGigastructuresClass>>,
    pub planet_entertainers: Box<Option<DiplomaticNetworking>>,
    pub planet_necro_apprentices: Box<Option<DiplomaticNetworking>>,
    pub armies: Box<Option<DiplomaticNetworking>>,
    pub planet_bio_trophies: Box<Option<DiplomaticNetworking>>,
    pub giga_megastructures: Box<Option<GigaMegastructures>>,
    pub leader_scientists: Box<Option<LeaderScientistsClass>>,
    pub giga_gigastructures: Box<Option<GigaGigastructuresClass>>,
    pub giga_kilostructures: Box<Option<PlanetMerchants>>,
    pub pop_category_workers: Box<Option<PopCategoryWorkers>>,
    pub leader_admirals: Box<Option<DiplomaticNetworking>>,
    pub pop_category_drones: Box<Option<PlanetMetallurgistsClass>>,
    pub planet_deposits: Box<Option<PlanetDeposits>>,
    pub pop_category_specialists: Box<Option<IncomePopCategoryRulers>>,
    pub pop_category_rulers: Box<Option<IncomePopCategoryRulers>>,
    pub ship_components: Box<Option<GigaKilostructures>>,
    pub ships: Box<Option<Ships>>,
    pub station_gatherers: Box<Option<Campaigns>>,
    pub planet_telepaths: Box<Option<DiplomaticNetworking>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GigaMegastructures {
    pub physics_research: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub society_research: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    pub rare_crystals: Box<Option<Number>>,
    pub engineering_research: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomeLeaderGenerals {
    pub minerals: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub consumer_goods: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurplePlanetArtisans {
    pub consumer_goods: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomePlanetFarmers {
    pub consumer_goods: Box<Option<Number>>,
    pub energy: Box<Option<Number>>,
    pub food: Box<Option<Number>>,
    pub unity: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomePlanetMiners {
    pub alloys: Box<Option<Number>>,
    pub acot_sr_light_matter: Box<Option<Number>>,
    pub acot_sr_stellarite: Box<Option<Number>>,
    pub minerals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomePopCategoryRulers {
    pub rare_crystals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopCategoryWorkers {
    pub food: Box<Option<Number>>,
    pub rare_crystals: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomeHighWaterMark {
    pub length: Box<Option<Number>>,
    pub history: Box<Option<StandardEconomyModule>>,
    pub current: Box<Option<VecOrMap<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardEconomyModule {
    pub resources: Box<Option<VecOrMap<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edict {
    pub edict: Box<Option<String>>,
    pub date: Box<Option<String>>,
    pub perpetual: Box<Option<String>>,
    pub start_date: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum EspionageManagerUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    EspionageManagerClass(EspionageManagerClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EspionageManagerClass {
    pub spy_networks: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryEthos {
    pub ethic: Box<Option<RequiredComponentUnion>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RequiredComponentUnion {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum EventsUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    EventsClass(EventsClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventsClass {
    pub poi: Box<Option<Vec<Poi>>>,
    pub event_chain: Box<Option<EventChain>>,
    pub anomalies: Box<Option<Vec<Number>>>,
    pub situations: Box<Option<Vec<Number>>>,
    pub special_project: Box<Option<SpecialProjectUnion>>,
    pub next_special_project_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventChain {
    pub event_chain: Box<Option<String>>,
    pub scope: Box<Option<EventChainScope>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventChainScope {
    pub root: Box<Box<Option<FromClass>>>,
    pub random: Box<Option<Vec<Number>>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub prev: Box<Option<PurplePrev>>,
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    pub random_allowed: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FromClass {
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub random: Box<Option<Vec<Number>>>,
    pub id: Box<Option<Number>>,
    pub random_allowed: Box<Option<String>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    pub from: Box<Box<Option<FromClass>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SavedEventTargetUnion {
    SavedEventTargetElement(SavedEventTargetElement),
    SavedEventTargetElementArray(Vec<SavedEventTargetElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedEventTargetElement {
    pub name: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub saved_event_target_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeVariables {
    pub local_wormholes_num: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurplePrev {
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub random: Box<Option<Vec<Number>>>,
    pub id: Box<Option<Number>>,
    pub random_allowed: Box<Option<String>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub root: Box<Box<Option<FromClass>>>,
    pub prev: Box<Box<Option<FromClass>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poi {
    pub id: Box<Option<String>>,
    pub event_chain: Box<Option<String>>,
    pub desc: Box<Option<String>>,
    pub location: Box<Option<String>>,
    pub scope: Box<Option<PoiScope>>,
    pub has_location: Box<Option<String>>,
    pub name: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoiScope {
    pub id: Box<Option<Number>>,
    pub random: Box<Option<Vec<Number>>>,
    pub from: Box<Box<Option<FromClass>>>,
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub random_allowed: Box<Option<String>>,
    pub root: Box<Box<Option<FromClass>>>,
    pub prev: Box<Box<Option<FromClass>>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SpecialProjectUnion {
    PurpleSpecialProject(PurpleSpecialProject),
    SpecialProjectElementArray(Vec<SpecialProjectElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecialProjectElement {
    pub id: Box<Option<Number>>,
    pub coordinate: Box<Option<Owner>>,
    pub debris: Box<Option<Number>>,
    pub days_left: Box<Option<Number>>,
    pub status: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleSpecialProject {
    pub status: Box<Option<String>>,
    pub ai_research_date: Box<Option<String>>,
    pub planet: Box<Option<Number>>,
    pub id: Box<Option<Number>>,
    pub scope: Box<Option<SpecialProjectScope>>,
    pub special_project: Box<Option<String>>,
    pub coordinate: Box<Option<Owner>>,
    pub convert_to: Box<Option<Number>>,
    pub species: Box<Option<Vec<SpeciesElement>>>,
    pub days_left: Box<Option<Number>>,
    pub debris: Box<Option<Number>>,
    pub ambient_object: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecialProjectScope {
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub random_allowed: Box<Option<String>>,
    pub root: Box<Box<Option<FromClass>>>,
    pub id: Box<Option<Number>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub prev: Box<Option<FluffyPrev>>,
    pub random: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffyPrev {
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub random_allowed: Box<Option<String>>,
    pub root: Box<Box<Option<FromClass>>>,
    pub id: Box<Option<Number>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub prev: Box<Box<Option<TentacledPrev>>>,
    pub random: Box<Option<Vec<Number>>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    pub variables: Box<Option<ScopeVariables>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TentacledPrev {
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub random_allowed: Box<Option<String>>,
    pub root: Box<Box<Option<FromClass>>>,
    pub id: Box<Option<Number>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub prev: Box<Box<Option<TentacledPrev>>>,
    pub random: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeciesElement {
    pub planet: Box<Option<Number>>,
    pub species: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Faction {
    pub needs_colony: Box<Option<String>>,
    pub auto_delete: Box<Option<String>>,
    pub aggro_range_measure_from: Box<Option<String>>,
    pub needs_border_access: Box<Option<String>>,
    pub aggro_range: Box<Option<Number>>,
    pub intel_effects_surveyed: Box<Option<String>>,
    pub generate_borders: Box<Option<String>>,
    pub hostile: Box<Option<String>>,
    pub space_creatures: Box<Option<String>>,
    pub hostile_when_attacked: Box<Option<String>>,
    pub primitive: Box<Option<String>>,
    pub neutral: Box<Option<String>>,
    pub copy_hostility: Box<Option<String>>,
    pub country: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FirstContactUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    FirstContactClass(FirstContactClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirstContactClass {
    pub contacts: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flag {
    pub background: Box<Option<Background>>,
    pub colors: Box<Option<Vec<String>>>,
    pub icon: Box<Option<Background>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Background {
    pub category: Box<Option<String>>,
    pub file: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryFlags {
    #[serde(rename = "Story7")]
    pub story7: Box<Option<Number>>,
    pub league_of_nations_achievement: Box<Option<Number>>,
    pub got_ownt_by_stellarite_once: Box<Option<Number>>,
    #[serde(rename = "Story6")]
    pub story6: Box<Option<Number>>,
    pub galcom_form_aye: Box<Option<Number>>,
    pub encountered_first_gateway: Box<Option<Number>>,
    pub sce_bombers: Box<Option<Number>>,
    pub acquired_light_matter: Box<Option<Number>>,
    pub oe_player: Box<Option<Number>>,
    pub first_contact_completed49: Box<Option<Number>>,
    pub tutorial_level_picked: Box<Option<Number>>,
    pub found_presapients: Box<Option<Number>>,
    pub know_dark_power_final: Box<Option<Number>>,
    #[serde(rename = "Story5")]
    pub story5: Box<Option<Number>>,
    pub recent_comms_timed: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub giga_host: Box<Option<Number>>,
    #[serde(rename = "Story8")]
    pub story8: Box<Option<Number>>,
    pub nsc_modmenu_warning_evt_fired: Box<Option<Number>>,
    pub giga_achievement_12: Box<Option<Number>>,
    pub giga_achievement_11: Box<Option<Number>>,
    pub encountered_first_wormhole: Box<Option<Number>>,
    pub giga_menu_page_00: Box<Option<Number>>,
    pub giga_nexus_tech_flag_set: Box<Option<Number>>,
    #[serde(rename = "Story4")]
    pub story4: Box<Option<Number>>,
    pub giga_achievement_37: Box<Option<Number>>,
    #[serde(rename = "Story2")]
    pub story2: Box<Option<Number>>,
    pub sce_railpods: Box<Option<Number>>,
    pub giga_achievement_36: Box<Option<Number>>,
    pub recent_first_contact_process_started: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub acot_finished_delta: Box<Option<Number>>,
    pub giga_achievement_13: Box<Option<Number>>,
    pub giga_achievement_15: Box<Option<Number>>,
    pub giga_achievement_40: Box<Option<Number>>,
    pub salvager_enclave_diplomacy_engaged: Box<Option<Number>>,
    pub giga_achievement_39: Box<Option<Number>>,
    pub giga_achievement_14: Box<Option<Number>>,
    #[serde(rename = "Story1")]
    pub story1: Box<Option<Number>>,
    pub ae_dark_power_delay: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub custom_start_screen: Box<Option<Number>>,
    #[serde(rename = "StoryFirst")]
    pub story_first: Box<Option<Number>>,
    pub first_contact_event: Box<Option<Number>>,
    pub stellarite_tech_enable: Box<Option<Number>>,
    pub shroudwalker_enclave_diplomacy_engaged: Box<Option<Number>>,
    pub giga_achievement_38: Box<Option<Number>>,
    pub has_encountered_other_empire: Box<Option<Number>>,
    pub outer_system_mining_stations_constructed: Box<Option<Number>>,
    pub found_toxic_terraform_candidate: Box<Option<Number>>,
    pub encountered_first_lgate: Box<Option<Number>>,
    pub materialist_science: Box<Option<Number>>,
    pub encountered_first_primitive: Box<Option<Number>>,
    pub dark_matter_found: Box<Option<Number>>,
    pub sce_interceptors: Box<Option<Number>>,
    pub first_alien_life: Box<Option<Number>>,
    pub first_colony: Box<Option<Number>>,
    #[serde(rename = "is_advanced_start_AI")]
    pub is_advanced_start_ai: Box<Option<Number>>,
    pub ehof_precursor_spawned: Box<Option<Number>>,
    pub first_contact_protocol_event_happened: Box<Option<Number>>,
    pub giga_ai_savings_init: Box<Option<Number>>,
    pub galcom_form_nay: Box<Option<Number>>,
    pub whatever_it_is_im_against_it_rejected_community: Box<Option<Number>>,
    pub upl_ongoing_generic: Box<Option<Number>>,
    pub sce_fighters: Box<Option<Number>>,
    pub volatile_motes_found: Box<Option<Number>>,
    pub favored_insights: Box<Option<Number>>,
    #[serde(rename = "anomaly_outcome_happened_anomaly.135")]
    pub anomaly_outcome_happened_anomaly_135: Box<Option<Number>>,
    #[serde(rename = "fircon_2010_fired")]
    pub fircon_2010_fired: Box<Option<Fircon2010Fired>>,
    #[serde(rename = "anomaly_outcome_happened_anomaly.335")]
    pub anomaly_outcome_happened_anomaly_335: Box<Option<Number>>,
    pub recent_envoy_event: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub built_observation_post: Box<Option<Number>>,
    #[serde(rename = "anomaly_outcome_happened_anomaly.6855")]
    pub anomaly_outcome_happened_anomaly_6855: Box<Option<Number>>,
    pub found_terraform_candidate: Box<Option<Number>>,
    pub sce_gunships: Box<Option<Number>>,
    pub found_giga_o_star_class: Box<Option<Number>>,
    pub discovered_site: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub first_contact_completed121: Box<Option<Number>>,
    pub has_conquer_other_homeworld: Box<Option<Number>>,
    pub recent_primitive_invasion: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub spiritualist_accept: Box<Option<Number>>,
    pub first_contact_completed76: Box<Option<Number>>,
    pub crystals_encountered: Box<Option<Number>>,
    pub shoot_to_kill_achievement: Box<Option<Number>>,
    pub first_contact_completed54: Box<Option<Number>>,
    pub dancing_fever_colony_event: Box<Option<Number>>,
    pub encountered_ruined_accelerator: Box<Option<Number>>,
    pub non_lithoid_subspecies: Box<Option<Number>>,
    pub exotic_gases_found: Box<Option<Number>>,
    pub amoeba_encountered: Box<Option<Number>>,
    pub first_contact_completed70: Box<Option<Number>>,
    pub living_metal_found: Box<Option<Number>>,
    pub sce_gatlings: Box<Option<Number>>,
    pub had_comet: Box<Option<Number>>,
    pub encountered_ruined_orbital: Box<Option<Number>>,
    pub has_negotiated_trade_deal: Box<Option<Number>>,
    pub encountered_ruined_orchid: Box<Option<Number>>,
    pub encountered_corinth: Box<Option<Number>>,
    pub encountered_ruined_kugelblitz: Box<Option<Number>>,
    pub rare_crystals_found: Box<Option<Number>>,
    pub first_contact_completed46: Box<Option<Number>>,
    pub first_contact_completed44: Box<Option<Number>>,
    pub encountered_tripledwarf: Box<Option<Number>>,
    pub zro_found: Box<Option<Number>>,
    pub with_great_power_achievement_locked: Box<Option<Number>>,
    pub first_contact_completed95: Box<Option<Number>>,
    pub first_contact_completed192: Box<Option<Number>>,
    pub encountered_menagerie: Box<Option<Number>>,
    pub first_contact_completed48: Box<Option<Number>>,
    pub encountered_ruined_orbitalsystem: Box<Option<Number>>,
    pub tiyanki_encountered: Box<Option<Number>>,
    pub first_contact_completed55: Box<Option<Number>>,
    pub giga_achievement_69: Box<Option<Number>>,
    pub tier_dm_start: Box<Option<Number>>,
    pub giga_fe_repeatables_modifier_applied: Box<Option<Number>>,
    pub has_built_or_repaired_megastructure: Box<Option<Number>>,
    pub already_setup_max_ast_artillery_points: Box<Option<Number>>,
    pub has_recently_built_asteroid_artillery: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub fallen_empire_4: Box<Option<Number>>,
    pub fallen_empire_2: Box<Option<Number>>,
    pub has_built_or_repaired_gigastructure: Box<Option<Number>>,
    pub fallen_empire_3: Box<Option<Number>>,
    pub fallen_empire_machine: Box<Option<Number>>,
    pub fallen_machine_empire_awaken_2: Box<Option<Number>>,
    pub fallen_empire_1: Box<Option<Number>>,
    pub giga_achievement_85: Box<Option<Number>>,
    pub giga_meopa_fe: Box<Option<Number>>,
    pub giga_achievement_62: Box<Option<Number>>,
    pub giga_pouchkinn_meopa_fe: Box<Option<Number>>,
    pub giga_achievement_54: Box<Option<Number>>,
    pub late_medieval_age: Box<Option<Number>>,
    pub iron_age: Box<Option<Number>>,
    pub bronze_age: Box<Option<Number>>,
    pub industrial_age: Box<Option<Number>>,
    pub machine_age: Box<Option<Number>>,
    pub humans_machine_age: Box<Option<Number>>,
    pub recent_awareness_gain: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub stone_age: Box<Option<Number>>,
    pub phanon_base_country: Box<Option<Number>>,
    pub acot_herculean_rumor_heard: Box<Option<Number>>,
    pub salvager_enclave_country: Box<Option<Number>>,
    pub marauder_1: Box<Option<Number>>,
    pub automated_dreadnought_country: Box<Option<Number>>,
    pub trader_enclave_country: Box<Option<Number>>,
    pub trader_enclave_country_2: Box<Option<Number>>,
    pub caravaneer_fleet2_country: Box<Option<Number>>,
    pub steam_age: Box<Option<Number>>,
    pub marauder_3: Box<Option<Number>>,
    pub artist_enclave_country: Box<Option<Number>>,
    pub polaris_country: Box<Option<Number>>,
    pub renaissance_age: Box<Option<Number>>,
    pub early_space_age: Box<Option<Number>>,
    pub renaissance_age_pops: Box<Option<Number>>,
    pub recently_advanced: Box<Option<Number>>,
    pub atomic_age: Box<Option<Number>>,
    pub atomic_age_pops: Box<Option<Number>>,
    pub blokkat_spawndate_set: Box<Option<Number>>,
    pub marauder_2: Box<Option<Number>>,
    pub caravaneer_home_country: Box<Option<Number>>,
    pub hulk_caravan_recent: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub recent_awareness_decay: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub guardian_scavenger_bot: Box<Option<Number>>,
    pub caravaneer_fleet1_country: Box<Option<Number>>,
    pub early_space_age_pops: Box<Option<Number>>,
    pub trader_enclave_country_3: Box<Option<Number>>,
    pub trader_enclave_country_1: Box<Option<Number>>,
    pub here_be_the_dragon: Box<Option<Number>>,
    pub here_be_the_dragon9: Box<Option<Number>>,
    pub curator_enclave_country: Box<Option<Number>>,
    pub fortress_country: Box<Option<Number>>,
    pub iron_age_pops: Box<Option<Number>>,
    pub shroudwalker_enclave_country: Box<Option<Number>>,
    pub created_robots: Box<Option<Number>>,
    pub lone_defender_country: Box<Option<Number>>,
    pub hillos_country: Box<Option<Number>>,
    pub lost_swarm: Box<Option<Number>>,
    pub steam_age_pops: Box<Option<Number>>,
    pub suppress_first_contact_comms: Box<Option<Number>>,
    pub chosen_empire: Box<Option<Number>>,
    pub acot_precursor_databank_country: Box<Option<Number>>,
    pub sofe_the_light: Box<Option<Number>>,
    pub custom_preftl_diplomacy: Box<Option<Number>>,
    pub flusion_primitives: Box<Option<Number>>,
    pub custom_preftl_espionage: Box<Option<Number>>,
    pub tech_frozen: Box<Option<Number>>,
    pub corrona_primitives: Box<Option<Number>>,
    pub corrona_age: Box<Option<Number>>,
    pub paluush_primitives: Box<Option<Number>>,
    pub giga_eaw_country: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsteroidSightedPreFtlGlobal {
    pub flag_days: Box<Option<Number>>,
    pub flag_date: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Fircon2010Fired {
    AsteroidSightedPreFtlGlobal(AsteroidSightedPreFtlGlobal),
    Integer(Number),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FleetTemplateManagerUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    FleetTemplateManagerClass(FleetTemplateManagerClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetTemplateManagerClass {
    pub fleet_template: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FleetsManagerUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    FleetsManagerClass(FleetsManagerClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetsManagerClass {
    pub owned_fleets: Box<Option<Vec<OwnedFleet>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OwnedFleet {
    pub ownership_status: Box<Option<String>>,
    pub debtor: Box<Option<Number>>,
    pub lease_period: Box<Option<Number>>,
    pub previous_owner: Box<Option<Number>>,
    pub fleet: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Government {
    pub authority: Box<Option<String>>,
    pub council_agenda: Box<Option<String>>,
    #[serde(rename = "type")]
    pub government_type: Box<Option<String>>,
    pub council_agenda_progress: Box<Option<Number>>,
    pub council_positions: Box<Option<Vec<Number>>>,
    pub civics: Box<Option<Vec<String>>>,
    pub council_agenda_cooldowns: Box<Option<CouncilAgendaCooldowns>>,
    pub unlocked_civic_council_slots: Box<Option<Number>>,
    pub origin: Box<Option<String>>,
    pub heir: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouncilAgendaCooldowns {
    pub council_agenda: Box<Option<String>>,
    pub start_date: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HoldingPlanet {
    pub key: Box<Option<String>>,
    pub value: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncomingAction {
    pub action_embassy: Box<Option<ActionEmbassy>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionEmbassy {
    #[serde(rename = "type")]
    pub action_embassy_type: Box<Option<String>>,
    pub timeout: Box<Option<String>>,
    pub num_favors: Box<Option<Number>>,
    pub recipient: Box<Option<Number>>,
    pub id: Box<Option<Number>>,
    pub actor: Box<Option<Number>>,
    pub waiting_for_vote: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryIntel {
    pub object: Box<Option<Number>>,
    pub hostile: Box<Option<Vec<Hostile>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hostile {
    pub owner: Box<Option<Number>>,
    pub coordinate: Box<Option<Position>>,
    pub military_power: Box<Option<Number>>,
    pub name: Box<Option<HostileName>>,
    pub has_boss: Box<Option<String>>,
    pub size: Box<Option<Vec<String>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostileName {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<StickyVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StickyVariable {
    pub value: Box<Option<FluffyValue>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffyValue {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<FluffyVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntelManager {
    pub intel: Box<Option<Vec<Vec<IntelUnion>>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IntelUnion {
    Integer(Number),
    IntelIntel(IntelIntel),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntelIntel {
    pub intel: Box<Option<Number>>,
    pub stale_intel: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LastAllianceName {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<LastAllianceNameVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LastAllianceNameVariable {
    pub key: Box<Option<String>>,
    pub value: Box<Option<LastKilledCountryName>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LastKilledCountryName {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<LastKilledCountryNameVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LastKilledCountryNameVariable {
    pub value: Box<Option<RulerTitleFemaleClass>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RulerTitleFemaleClass {
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modules {
    pub all_technology_module: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub standard_expansion_module: Box<Option<StandardExpansionModuleUnion>>,
    pub standard_species_rights_module: Box<Option<StandardSpeciesRightsModule>>,
    pub standard_economy_module: Box<Option<StandardEconomyModule>>,
    pub standard_leader_module: Box<Option<StandardLeaderModule>>,
    pub standard_event_module: Box<Option<StandardEventModuleUnion>>,
    pub exclusive_diplomacy_module: Box<Option<ExclusiveDiplomacyModuleUnion>>,
    pub standard_trade_routes_module: Box<Option<StandardTradeRoutesModuleUnion>>,
    pub standard_pop_factions_module: Box<Option<StandardPopFactionsModule>>,
    pub standard_diplomacy_module: Box<Option<StandardDiplomacyModule>>,
    pub standard_technology_module: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub tiered_technology_module: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub basic_technology_module: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ExclusiveDiplomacyModuleUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    ExclusiveDiplomacyModuleClass(ExclusiveDiplomacyModuleClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExclusiveDiplomacyModuleClass {
    pub borders: Box<Option<Number>>,
    pub rivals: Box<Option<Vec<Number>>>,
    pub casus_belli: Box<Option<Vec<CasusBelli>>>,
    pub can_receive: Box<Option<Vec<String>>>,
    pub can_send: Box<Option<Vec<String>>>,
    pub rival: Box<Option<Number>>,
    pub contact_rule: Box<Option<String>>,
    pub closed_borders: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CasusBelli {
    pub owner: Box<Option<Number>>,
    pub country: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub casus_belli_type: Box<Option<String>>,
    pub days_left: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardDiplomacyModule {
    pub non_aggression_pledge: Box<Option<Number>>,
    pub migration_pacts: Box<Option<Number>>,
    pub borders: Box<Option<Number>>,
    pub contact_rule: Box<Option<String>>,
    pub casus_belli: Box<Option<Vec<CasusBelli>>>,
    pub commercial_pact: Box<Option<Number>>,
    pub research_agreement: Box<Option<Number>>,
    pub closed_borders: Box<Option<Number>>,
    pub defensive_pact: Box<Option<Number>>,
    pub rivals: Box<Option<Vec<Number>>>,
    pub can_receive: Box<Option<Vec<String>>>,
    pub can_send: Box<Option<Vec<String>>>,
    pub rival: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StandardEventModuleUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    StandardEventModuleClass(StandardEventModuleClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardEventModuleClass {
    pub delayed_event: Box<Option<DelayedEventUnion>>,
    pub accumulated_anomaly_spawn_chance: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DelayedEventUnion {
    DelayedEventElement(DelayedEventElement),
    DelayedEventElementArray(Vec<DelayedEventElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelayedEventElement {
    pub event: Box<Option<String>>,
    pub scope: Box<Box<Option<FromClass>>>,
    pub days: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StandardExpansionModuleUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    StandardExpansionModuleClass(StandardExpansionModuleClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardExpansionModuleClass {
    pub expansion_list: Box<Option<Vec<ExpansionList>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpansionList {
    pub target_planet: Box<Option<Number>>,
    pub construction_queue: Box<Option<Number>>,
    pub name: Box<Option<LastKilledCountryName>>,
    pub species_index: Box<Option<Number>>,
    pub ship_design: Box<Option<Number>>,
    pub construction_queue_item: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardLeaderModule {
    pub leaders: Box<Option<Vec<Number>>>,
    pub enabled: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardPopFactionsModule {
    pub potential_count: Box<Option<Vec<Number>>>,
    pub total_faction_members_power: Box<Option<Number>>,
    pub last_created: Box<Option<String>>,
    pub total_faction_members: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardSpeciesRightsModule {
    #[serde(rename = "default")]
    pub standard_species_rights_module_default: Box<Option<BuiltSpecies>>,
    pub enabled: Box<Option<String>>,
    pub built_species: Box<Option<BuiltSpecies>>,
    pub primary: Box<Option<BuiltSpecies>>,
    pub species_rights: Box<Option<Vec<BuiltSpecies>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuiltSpecies {
    pub living_standard: Box<Option<String>>,
    pub last_changed_citizenship_type: Box<Option<String>>,
    pub last_changed_military_service_type: Box<Option<String>>,
    pub former_military_service: Box<Option<String>>,
    pub military_service: Box<Option<String>>,
    pub slavery: Box<Option<String>>,
    pub colonization_control: Box<Option<String>>,
    pub population_control: Box<Option<String>>,
    pub last_changed_population_control: Box<Option<String>>,
    pub citizenship: Box<Option<String>>,
    pub former_population_control: Box<Option<String>>,
    pub migration_control: Box<Option<String>>,
    pub species_index: Box<Option<Number>>,
    pub former_citizenship: Box<Option<String>>,
    pub last_changed_living_standard: Box<Option<String>>,
    pub purge: Box<Option<String>>,
    pub former_slavery: Box<Option<String>>,
    pub former_purge: Box<Option<String>>,
    pub last_changed_purge_type: Box<Option<String>>,
    pub last_changed_colonization_control: Box<Option<String>>,
    pub last_changed_migration_control: Box<Option<String>>,
    pub last_changed_slavery_type: Box<Option<String>>,
    pub former_living_standard: Box<Option<String>>,
    pub former_migration_control: Box<Option<String>>,
    pub former_colonization_control: Box<Option<String>>,
    pub last_changed_value: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StandardTradeRoutesModuleUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    StandardTradeRoutesModuleClass(StandardTradeRoutesModuleClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandardTradeRoutesModuleClass {
    pub internal: Box<Option<Vec<Number>>>,
    pub last_month: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleName {
    pub key: Box<Option<String>>,
    pub literal: Box<Option<String>>,
    pub variables: Box<Option<Vec<IndigoVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IndigoVariable {
    pub value: Box<Option<LastAllianceName>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegnalNumber {
    pub regnal_name: Box<Option<RulerTitleFemaleClass>>,
    pub regnal_number: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelationsManager {
    pub relation: Box<Option<Vec<Relation>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relation {
    pub owner: Box<Option<Number>>,
    pub modifier: Box<Option<ModifierUnion>>,
    pub relation_last_month: Box<Option<Number>>,
    pub country: Box<Option<Number>>,
    pub communications: Box<Option<String>>,
    pub relation_current: Box<Option<Number>>,
    pub contact: Box<Option<String>>,
    pub diplo_action_dates: Box<Option<Vec<String>>>,
    pub border_range: Box<Option<Number>>,
    pub num_favors: Box<Option<Number>>,
    pub borders: Box<Option<String>>,
    pub closed_borders: Box<Option<String>>,
    pub is_rival: Box<Option<String>>,
    pub rival_date: Box<Option<String>>,
    pub pre_communications_name: Box<Option<LastAllianceName>>,
    pub hostile: Box<Option<String>>,
    pub trust: Box<Option<Number>>,
    pub embassy: Box<Option<String>>,
    pub envoy_opinion_positive: Box<Option<Number>>,
    pub migration_access: Box<Option<String>>,
    pub non_aggression_pledge: Box<Option<String>>,
    pub research_agreement: Box<Option<String>>,
    pub foreign_envoys: Box<Option<Vec<Number>>>,
    pub commercial_pact: Box<Option<String>>,
    pub defensive_pact: Box<Option<String>>,
    pub alliance: Box<Option<String>>,
    pub envoy_opinion_negative: Box<Option<Number>>,
    pub friendly: Box<Option<String>>,
    pub neutral: Box<Option<String>>,
    pub killed_ships: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ModifierUnion {
    ModifierElement(ModifierElement),
    ModifierElementArray(Vec<ModifierElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModifierElement {
    pub value: Box<Option<Number>>,
    pub modifier: Box<Option<String>>,
    pub start_date: Box<Option<String>>,
    pub decay: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sectors {
    pub owned: Box<Option<Vec<Number>>>,
    pub monthly_transfer: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub resources: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryShipDesignCollection {
    pub ship_design: Box<Option<Vec<Number>>>,
    pub auto_gen_design: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TechStatus {
    pub level: Box<Option<GalacticObjectUnion>>,
    pub auto_researching_physics: Box<Option<String>>,
    pub auto_researching_engineering: Box<Option<String>>,
    pub society_queue: Box<Option<Vec<EngineeringQueueElement>>>,
    pub last_increased_tech: Box<Option<String>>,
    pub stored_techpoints: Box<Option<Vec<Number>>>,
    pub potential: Box<Option<VecOrMap<String>>>,
    pub auto_researching_society: Box<Option<String>>,
    pub alternatives: Box<Option<Alternatives>>,
    pub technology: Box<Option<RequiredComponentUnion>>,
    pub engineering_queue: Box<Option<Vec<EngineeringQueueElement>>>,
    pub physics_queue: Box<Option<Vec<EngineeringQueueElement>>>,
    pub always_available_tech: Box<Option<RequiredComponentUnion>>,
    pub stored_techpoints_for_tech: Box<Option<StoredTechpointsForTech>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alternatives {
    pub society: Box<Option<Vec<String>>>,
    pub engineering: Box<Option<Vec<String>>>,
    pub physics: Box<Option<Vec<String>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineeringQueueElement {
    pub technology: Box<Option<String>>,
    pub date: Box<Option<String>>,
    pub progress: Box<Option<Number>>,
    pub special_project: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum GalacticObjectUnion {
    Integer(Number),
    IntegerArray(Vec<Number>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredTechpointsForTech {
    pub tech_crystal_armor_1: Box<Option<Number>>,
    pub tech_regenerative_hull_tissue: Box<Option<Number>>,
    pub tech_amoeba_strike_craft_1: Box<Option<Number>>,
    pub tech_strike_craft_1: Box<Option<Number>>,
    pub tech_ship_armor_2: Box<Option<Number>>,
    pub tech_space_mining_2: Box<Option<Number>>,
    pub tech_space_whale_weapon_1: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerraIncognita {
    pub systems: Box<Option<Vec<Number>>>,
    pub size: Box<Option<Number>>,
    pub data: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountryTimedModifier {
    pub items: Box<Option<Vec<PurpleItem>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleItem {
    pub modifier: Box<Option<String>>,
    pub days: Box<Option<Number>>,
    pub multiplier: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DebrisOrValue {
    Debris(Debris),
    Nil(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Debris {
    pub resources: Box<Option<Vec<Vec<ResourceUnion>>>>,
    pub component: Box<Option<Vec<String>>>,
    pub date: Box<Option<String>>,
    pub coordinate: Box<Option<Position>>,
    pub ship_size: Box<Option<Vec<String>>>,
    pub from_country: Box<Option<Number>>,
    pub country: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ResourceUnion {
    Integer(Number),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DepositValue {
    DepositClass(DepositClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositClass {
    pub planet: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub deposit_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EspionageOperations {
    pub operations: Option<VecOrMap<OperationValue>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OperationValue {
    OperationClass(OperationClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationClass {
    pub target: Option<DataClass>,
    pub spy_network: Option<Number>,
    #[serde(rename = "type")]
    pub operation_type: Option<String>,
    pub index: Option<Number>,
    pub last_roll: Option<Number>,
    pub days_left: Option<Number>,
    pub difficulty: Option<Number>,
    pub info: Option<Number>,
    pub log: Option<Vec<OperationLog>>,
    pub assigned_assets: Option<Vec<Number>>,
    pub completed: Option<Vec<OperationCompleted>>,
    pub events: Option<Vec<OperationEvent>>,
    pub flags: Option<OperationFlags>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataClass {
    #[serde(rename = "type")]
    pub target_type: Option<String>,
    pub id: Option<Number>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationCompleted {
    pub date: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationEvent {
    pub expired: Option<String>,
    pub event_id: Option<String>,
    pub scope: Option<PurpleScope>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub effect: Option<String>,
    pub picture: Option<String>,
    pub index: Option<Number>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleScope {
    #[serde(rename = "type")]
    pub scope_type: Option<String>,
    pub id: Option<Number>,
    pub random: Option<Vec<Number>>,
    pub random_allowed: Option<String>,
    pub from: Box<Option<EventRoot>>,
    pub prev: Box<Option<EventRoot>>,
    pub saved_event_target: Option<SavedEventTarget>,
    pub root: Box<Option<EventRoot>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventRoot {
    #[serde(rename = "type")]
    pub scope_type: Option<String>,
    pub id: Option<Number>,
    pub random: Option<Vec<Number>>,
    pub random_allowed: Option<String>,
    pub from: Option<From>,
    pub saved_event_target: Option<SavedEventTarget>,
    pub root: Box<Option<EventRoot>>,
    pub prev: Option<PurpleScope>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedEventTarget {
    #[serde(rename = "type")]
    pub saved_event_target_type: Option<String>,
    pub id: Option<Number>,
    pub name: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationFlags {
    pub operation_random_event_fired: Option<AsteroidSightedPreFtlGlobal>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationLog {
    pub title: Option<String>,
    pub tooltip: Option<String>,
    pub roll: Option<Number>,
    pub skill: Option<Number>,
    pub info: Option<Number>,
    pub difficulty: Option<f64>,
    pub date: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Federation {
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FederationProgression {
    pub base_cohesion: Box<Option<Number>>,
    pub action_settings: Box<Option<ActionSettings>>,
    pub experience: Box<Option<Number>>,
    pub envoy: Box<Option<Vec<Number>>>,
    pub succession_term: Box<Option<String>>,
    pub last_succession_date: Box<Option<String>>,
    pub flags: Box<Option<FederationProgressionFlags>>,
    pub research_agreement: Box<Option<String>>,
    pub laws: Box<Option<Laws>>,
    pub expired: Box<Option<String>>,
    pub only_leader_builds_fleets: Box<Option<String>>,
    pub allow_subjects_to_join: Box<Option<String>>,
    pub failed_vote_half_price: Box<Option<String>>,
    pub free_migration: Box<Option<String>>,
    pub federation_type: Box<Option<String>>,
    pub levels: Box<Option<Number>>,
    pub perks: Box<Option<Vec<Perk>>>,
    pub succession_type: Box<Option<String>>,
    pub research_sharing: Box<Option<String>>,
    pub timed_modifier: Box<Option<FederationProgressionTimedModifier>>,
    pub cohesion: Box<Option<Number>>,
    pub equal_voting_power: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionSettings {
    pub action_invite_to_federation: Box<Option<Action>>,
    pub action_declare_war: Box<Option<Action>>,
    pub action_ask_to_leave_federation: Box<Option<Action>>,
    pub action_kick_from_federation: Box<Option<Action>>,
    pub action_offer_peace: Box<Option<Action>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub acceptance_type: Box<Option<String>>,
    pub vote_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FederationProgressionFlags {
    pub enable_federation_cooldowns: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Laws {
    pub law_category_allow_subjects_to_join: Box<Option<String>>,
    pub law_category_separate_treaties: Box<Option<String>>,
    pub law_category_succession_power: Box<Option<String>>,
    pub law_category_free_migration: Box<Option<String>>,
    pub law_category_voting_weight: Box<Option<String>>,
    pub law_category_succession_term: Box<Option<String>>,
    pub law_category_kick_members_vote: Box<Option<String>>,
    pub law_category_invite_members_vote: Box<Option<String>>,
    pub law_category_fleet_contribution: Box<Option<String>>,
    pub law_category_centralization: Box<Option<String>>,
    pub law_category_build_fleets: Box<Option<String>>,
    pub law_category_succession_type: Box<Option<String>>,
    pub law_category_war_declaration_vote: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Perk {
    pub level: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub perk_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FederationProgressionTimedModifier {
    pub items: Box<Option<Vec<FluffyItem>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffyItem {
    pub modifier: Box<Option<String>>,
    pub days: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndecentVariable {
    pub key: Box<Option<String>>,
    pub value: Box<Option<TentacledValue>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TentacledValue {
    pub variables: Box<Option<Vec<IndigoVariable>>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct The0ShipDesignCollection {
    pub ship_design: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirstContacts {
    pub contacts: Box<Option<VecOrMap<Contact>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    pub status: Box<Option<String>>,
    pub country: Box<Option<Number>>,
    pub location: Box<Option<Number>>,
    pub completed: Box<Option<Vec<Completed>>>,
    pub last_roll: Box<Option<Number>>,
    pub clues: Box<Option<Number>>,
    pub owner: Box<Option<Number>>,
    pub flags: Box<Option<ContactFlags>>,
    pub leader: Box<Option<Number>>,
    pub stage: Box<Option<String>>,
    pub difficulty: Box<Option<Number>>,
    pub days_left: Box<Option<Number>>,
    pub events: Box<Option<Vec<Event>>>,
    pub date: Box<Option<String>>,
    pub name: Box<Option<LastAllianceName>>,
    pub event: Box<Option<Event>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Completed {
    pub stage: Box<Option<String>>,
    pub date: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub expired: Box<Option<String>>,
    pub event_id: Box<Option<String>>,
    pub picture: Box<Option<String>>,
    pub index: Box<Option<Number>>,
    pub scope: Box<Option<EventScope>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventScope {
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub random_allowed: Box<Option<String>>,
    pub root: Box<Option<ScopeRoot>>,
    pub id: Box<Option<Number>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub prev: Box<Box<Option<TentacledPrev>>>,
    pub random: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeRoot {
    #[serde(rename = "type")]
    pub root_type: Box<Option<String>>,
    pub random: Box<Option<Vec<Number>>>,
    pub id: Box<Option<Number>>,
    pub random_allowed: Box<Option<String>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub root: Box<Box<Option<RootRoot>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRoot {
    pub root: Box<Box<Option<RootRoot>>>,
    pub random: Box<Option<Vec<Number>>>,
    pub random_allowed: Box<Option<String>>,
    pub from: Box<Box<Option<FromClass>>>,
    #[serde(rename = "type")]
    pub root_type: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactFlags {
    pub enclave_first_contact: Box<Option<Number>>,
    pub crystals_first_contact: Box<Option<Number>>,
    pub amoeba_first_contact: Box<Option<Number>>,
    pub drones_first_contact: Box<Option<Number>>,
    pub tiyanki_first_contact: Box<Option<Number>>,
    pub marauders_first_contact: Box<Option<Number>>,
    pub caravaneers_first_contact: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FleetValue {
    PurpleFleet(PurpleFleet),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleFleet {
    pub civilian: Box<Option<String>>,
    pub ground_support_stance: Box<Option<String>>,
    pub combat: Box<Option<Combat>>,
    pub movement_manager: Box<Option<VecOrStruct<MovementManager>>>,
    pub current_order: Box<Option<VecOrStruct<CurrentOrder>>>,
    pub is_cloaked: Box<Option<String>>,
    pub fleet_stats: Box<Option<FleetStats>>,
    pub cached_disengaged_ships: Box<Option<Number>>,
    pub hit_points: Box<Option<Number>>,
    pub ships: Box<Option<Vec<Number>>>,
    pub mobile: Box<Option<String>>,
    pub military_power: Box<Option<Number>>,
    pub fleet_stance: Box<Option<String>>,
    pub name: Box<Option<FleetName>>,
    pub mia_from: Box<Option<Position>>,
    pub cached_killed_ships: Box<Option<Number>>,
    pub cached_disabled_ships: Box<Option<Number>>,
    pub order_id: Box<Option<Number>>,
    pub cached_combined_removed_ships: Box<Option<Number>>,
    pub order: Box<Option<OrderClass>>,
    pub station: Box<Option<String>>,
    pub fleet_template: Box<Option<Number>>,
    pub settings: Box<Option<Settings>>,
    pub auto_movement: Box<Option<AutoMovement>>,
    pub aggro_range_measure_from: Box<Option<Number>>,
    pub aggro_range: Box<Option<Number>>,
    pub action_initialized: Box<Option<String>>,
    pub actions: Box<Option<VecOrStruct<Actions>>>,
    pub flags: Box<Option<VecOrMap<Number>>>,
    pub strike_craft: Box<Option<FleetStrikeCraft>>,
    pub experience: Box<Option<Experience>>,
    pub missile: Box<Option<VecOrStruct<FleetMissile>>>,
    pub timed_modifier: Box<Option<FederationProgressionTimedModifier>>,
    pub mission: Box<Option<VecOrStruct<Mission>>>,
    pub incoming_merges: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actions {
    pub repeat: Box<Option<Repeat>>,
    pub wait: Box<Option<CurrentWait>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repeat {
    pub index: Box<Option<Number>>,
    pub data: Box<Option<Data>>,
    pub scope: Box<Option<RepeatScope>>,
    pub iterations: Box<Option<Number>>,
    pub current: Box<Option<RepeatCurrent>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepeatCurrent {
    pub find_planet: Box<Option<CurrentFindPlanet>>,
    pub wait: Box<Option<CurrentWait>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentFindPlanet {
    pub scope: Box<Option<FindPlanetScope>>,
    pub planet: Box<Option<Number>>,
    pub index: Box<Option<Number>>,
    pub current: Box<Option<FindPlanetCurrent>>,
    pub data: Box<Option<FindPlanetElement>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindPlanetCurrent {
    pub orbit_planet: Box<Option<OrbitPlanet>>,
    pub move_to: Box<Option<MoveTo>>,
    pub wait: Box<Option<CurrentWait>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveTo {
    pub coordinate: Box<Option<Position>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitPlanet {
    pub orbitable: Box<Option<StarClass>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StarClass {
    pub planet: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentWait {
    pub time: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindPlanetElement {
    pub method: Box<Option<String>>,
    pub trigger: Box<Option<String>>,
    pub found_planet: Box<Option<DataFoundPlanet>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataFoundPlanet {
    pub orbit_planet: Box<Option<String>>,
    pub move_to: Box<Option<String>>,
    pub wait: Box<Option<FoundPlanetWait>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FoundPlanetWait {
    Integer(Number),
    WaitWait(WaitWait),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaitWait {
    pub duration: Box<Option<Number>>,
    pub random: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindPlanetScope {
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub random: Box<Option<Vec<Number>>>,
    pub root: Box<Option<From>>,
    pub id: Box<Option<Number>>,
    pub random_allowed: Box<Option<String>>,
    pub from: Box<Option<From>>,
    pub prev: Box<Option<StickyPrev>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct From {
    pub random: Box<Option<Vec<Number>>>,
    pub random_allowed: Box<Option<String>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    #[serde(rename = "type")]
    pub from_type: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    pub prev: Box<Box<Option<FromClass>>>,
    pub from: Box<Box<Option<FromClass>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StickyPrev {
    pub id: Box<Option<Number>>,
    pub random: Box<Option<Vec<Number>>>,
    pub random_allowed: Box<Option<String>>,
    pub root: Box<Option<From>>,
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub from: Box<Option<From>>,
    pub prev: Box<Option<IndigoPrev>>,
    pub saved_event_target: Box<Option<Vec<SavedEventTargetElement>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndigoPrev {
    pub saved_event_target: Box<Option<Vec<SavedEventTargetElement>>>,
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub random_allowed: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    pub random: Box<Option<Vec<Number>>>,
    pub from: Box<Box<Option<FromClass>>>,
    pub root: Box<Box<Option<FromClass>>>,
    pub prev: Box<Option<PoiScope>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub find_planet: Box<Option<FindPlanetUnion>>,
    pub max_iterations: Box<Option<Number>>,
    pub wait: Box<Option<DataWait>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FindPlanetUnion {
    FindPlanetElementArray(Vec<FindPlanetElement>),
    PurpleFindPlanet(PurpleFindPlanet),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleFindPlanet {
    pub found_planet: Box<Option<PurpleFoundPlanet>>,
    pub method: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleFoundPlanet {
    pub move_to: Box<Option<String>>,
    pub orbit_planet: Box<Option<String>>,
    pub wait: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DataWait {
    Integer(Number),
    IntegerArray(Vec<Number>),
    WaitWait(WaitWait),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepeatScope {
    pub id: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub scope_type: Box<Option<String>>,
    pub from: Box<Option<From>>,
    pub prev: Box<Option<IndecentPrev>>,
    pub root: Box<Option<From>>,
    pub random: Box<Option<Vec<Number>>>,
    pub random_allowed: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndecentPrev {
    pub prev: Box<Option<HilariousPrev>>,
    pub from: Box<Option<From>>,
    pub random_allowed: Box<Option<String>>,
    pub random: Box<Option<Vec<Number>>>,
    pub root: Box<Option<From>>,
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    pub saved_event_target: Box<Option<Vec<SavedEventTargetElement>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HilariousPrev {
    pub random_allowed: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub prev_type: Box<Option<String>>,
    pub saved_event_target: Box<Option<SavedEventTargetUnion>>,
    pub random: Box<Option<Vec<Number>>>,
    pub root: Box<Option<From>>,
    pub prev: Box<Option<PoiScope>>,
    pub from: Box<Option<From>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoMovement {
    #[serde(rename = "type")]
    pub auto_movement_type: Box<Option<String>>,
    pub has_arrived: Box<Option<String>>,
    pub clear_on_new_orders: Box<Option<String>>,
    pub auto_move_target: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Combat {
    pub formation: Box<Option<CombatFormation>>,
    pub coordinate: Box<Option<Position>>,
    pub formation_pos: Box<Option<FormationPos>>,
    pub start_coordinate: Box<Option<Position>>,
    pub start_date: Box<Option<String>>,
    pub in_combat_with: Box<Option<Vec<InCombatWith>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombatFormation {
    pub root: Box<Option<Number>>,
    pub parent: Box<Option<Vec<Number>>>,
    pub ships: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormationPos {
    pub forward_x: Box<Option<Number>>,
    pub y: Box<Option<Number>>,
    pub forward_y: Box<Option<Number>>,
    pub x: Box<Option<Number>>,
    pub rotation: Box<Option<Number>>,
    pub speed: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InCombatWith {
    pub fleet: Box<Option<Number>>,
    pub debris: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentOrder {
    pub assist_research_order: Box<Option<AssistResearchOrder>>,
    pub build_orbital_station_order: Box<Option<BuildOrbitalStationOrder>>,
    pub survey_planet_order: Box<Option<Order>>,
    pub research_anomaly_order: Box<Option<Order>>,
    pub orbit_planet_order: Box<Option<CurrentOrderOrbitPlanetOrder>>,
    pub collect_data_fleet_order: Box<Option<EtOrder>>,
    pub excavate_archaeological_site_fleet_order: Box<Option<EtOrder>>,
    pub move_to_system_point_order: Box<Option<CurrentOrderMoveToSystemPointOrder>>,
    pub follow_order: Box<Option<FollowOrder>>,
    pub aggressive_stance_fleet_order: Box<Option<AggressiveStanceFleetOrder>>,
    pub return_fleet_order: Box<Option<ReturnFleetOrder>>,
    pub colonize_planet_order: Box<Option<Order>>,
    pub merge_fleet_order: Box<Option<MergeFleetOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggressiveStanceFleetOrder {
    pub return_coordinate: Box<Option<ReturnCoordinateClass>>,
    pub target: Box<Option<Number>>,
    pub order_id: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    pub bombard_target: Box<Option<Number>>,
    pub sub_order: Box<Option<AggressiveStanceFleetOrderSubOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnCoordinateClass {
    pub coordinate: Box<Option<Position>>,
    pub target: Box<Option<Owner>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggressiveStanceFleetOrderSubOrder {
    pub follow_order: Box<Option<FollowOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FollowOrder {
    pub can_reach: Box<Option<String>>,
    pub fleet: Box<Option<Number>>,
    pub attack_when_in_range: Box<Option<String>>,
    pub coordinate: Box<Option<Position>>,
    pub order_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssistResearchOrder {
    pub can_reach: Box<Option<String>>,
    pub planet: Box<Option<Number>>,
    pub order_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildOrbitalStationOrder {
    pub ship_design: Box<Option<Number>>,
    pub progress: Box<Option<Number>>,
    pub cost: Box<Option<Number>>,
    pub planet: Box<Option<Number>>,
    pub in_progress: Box<Option<String>>,
    pub class: Box<Option<String>>,
    pub order_id: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    pub resources: Box<Option<BuildOrbitalStationOrderResources>>,
    pub sub_order: Box<Option<BuildOrbitalStationOrderSubOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildOrbitalStationOrderResources {
    pub minerals: Box<Option<Number>>,
    pub alloys: Box<Option<Number>>,
    pub influence: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildOrbitalStationOrderSubOrder {
    pub move_to_system_point_order: Box<Option<SubOrderMoveToSystemPointOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubOrderMoveToSystemPointOrder {
    pub coordinate: Box<Option<Position>>,
    pub can_reach: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EtOrder {
    pub progress: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    pub order_id: Box<Option<Number>>,
    pub special_project: Box<Option<Number>>,
    pub archaeological_site: Box<Option<Number>>,
    pub planet: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub planet: Box<Option<Number>>,
    pub progress: Box<Option<Number>>,
    pub order_id: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    pub sub_order: Box<Option<ColonizePlanetOrderSubOrder>>,
    pub name: Box<Option<SpeciesAdjectiveClass>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeciesAdjectiveClass {
    pub key: Box<Option<String>>,
    pub variables: Box<Option<Vec<LastKilledCountryNameVariable>>>,
    pub literal: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColonizePlanetOrderSubOrder {
    pub orbit_planet_order: Box<Option<CurrentOrderOrbitPlanetOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentOrderOrbitPlanetOrder {
    pub star: Box<Option<StarClass>>,
    pub merge_fleet: Box<Option<String>>,
    pub sub_order: Box<Option<BuildOrbitalStationOrderSubOrder>>,
    pub orbitable: Box<Option<StarClass>>,
    pub can_reach: Box<Option<String>>,
    pub order_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergeFleetOrder {
    pub order_id: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    pub ordered_by: Box<Option<Number>>,
    pub sub_order: Box<Option<AggressiveStanceFleetOrderSubOrder>>,
    pub target: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentOrderMoveToSystemPointOrder {
    pub coordinate: Box<Option<Position>>,
    pub order_id: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnFleetOrder {
    pub order_id: Box<Option<Number>>,
    pub can_reach: Box<Option<String>>,
    pub exclude_allied: Box<Option<String>>,
    pub sub_order: Box<Option<ReturnFleetOrderSubOrder>>,
    pub home_base: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnFleetOrderSubOrder {
    pub orbit_planet_order: Box<Option<PurpleOrbitPlanetOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleOrbitPlanetOrder {
    pub can_reach: Box<Option<String>>,
    pub merge_fleet: Box<Option<String>>,
    pub orbitable: Box<Option<BuildableColonyShipOrbitable>>,
    pub star: Box<Option<StarClass>>,
    pub sub_order: Box<Option<BuildOrbitalStationOrderSubOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub ships_killed: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetStats {
    pub combat_stats: Box<Option<CombatStats>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombatStats {
    pub fleet: Box<Option<CombatStatsFleetUnion>>,
    pub date: Box<Option<String>>,
    pub enemy: Box<Option<Vec<FleetElement>>>,
    pub damage_incoming: Box<Option<Vec<DamageIng>>>,
    pub hit_ratio_outgoing: Box<Option<Vec<HitRatioIng>>>,
    pub hit_ratio_incoming: Box<Option<Vec<HitRatioIng>>>,
    pub damage_outgoing: Box<Option<Vec<DamageIng>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DamageIng {
    pub fleet_index: Box<Option<Number>>,
    pub damage_armor: Box<Option<Number>>,
    pub base_damage_armor: Box<Option<Number>>,
    pub template: Box<Option<String>>,
    pub base_damage_hitpoints: Box<Option<Number>>,
    pub damage_hitpoints: Box<Option<Number>>,
    pub damage_shields: Box<Option<Number>>,
    pub base_damage_shields: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetElement {
    pub ship_size_name: Box<Option<Vec<String>>>,
    pub country: Box<Option<Number>>,
    pub country_name: Box<Option<String>>,
    pub empire_flag: Box<Option<Flag>>,
    pub ship_class: Box<Option<String>>,
    pub ship_size_key: Box<Option<Vec<String>>>,
    pub ship_size_count: Box<Option<Vec<Number>>>,
    pub fleet: Box<Option<Number>>,
    pub fleet_name: Box<Option<String>>,
    pub ship_size_count_lost: Box<Option<Vec<Number>>>,
    pub leader: Box<Option<FleetLeader>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetLeader {
    pub skill_cap: Box<Option<Number>>,
    #[serde(rename = "trait")]
    pub leader_trait: Box<Option<String>>,
    pub skill: Box<Option<Number>>,
    pub name: Box<Option<String>>,
    pub class: Box<Option<String>>,
    pub experience: Box<Option<Number>>,
    pub design: Box<Option<LeaderDesign>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderDesign {
    pub attachment: Box<Option<Number>>,
    pub leader_class: Box<Option<String>>,
    pub gender: Box<Option<String>>,
    pub name: Box<Option<FluffyName>>,
    pub clothes: Box<Option<Number>>,
    pub portrait: Box<Option<String>>,
    pub texture: Box<Option<Number>>,
    #[serde(rename = "trait")]
    pub design_trait: Box<Option<String>>,
    pub heir_title_female: Box<Option<PluralClass>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffyName {
    pub use_full_regnal_name: Box<Option<String>>,
    pub full_names: Box<Option<LastKilledCountryName>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CombatStatsFleetUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    FleetElement(FleetElement),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HitRatioIng {
    pub hits: Box<Option<Number>>,
    pub misses: Box<Option<Number>>,
    pub ship_design: Box<Option<Number>>,
    pub template: Box<Option<String>>,
    pub evades: Box<Option<Number>>,
    pub fleet_index: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetMissile {
    pub missile: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mission {
    pub mission: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovementManager {
    pub coordinate: Box<Option<Position>>,
    pub formation: Box<Option<MovementManagerFormation>>,
    pub time_since_last_path_update: Box<Option<Number>>,
    pub target_coordinate: Box<Option<Position>>,
    pub state: Box<Option<String>>,
    pub orbit: Box<Option<OrbitUnion>>,
    pub target: Box<Option<ReturnCoordinateClass>>,
    pub path: Box<Option<MovementManagerPath>>,
    pub last_ftl_jump: Box<Option<LastFtlJump>>,
    pub ftl_total_windup: Box<Option<Number>>,
    pub ftl_windup: Box<Option<Number>>,
    pub custom_formation: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovementManagerFormation {
    #[serde(rename = "type")]
    pub formation_type: Box<Option<String>>,
    pub scale: Box<Option<Number>>,
    pub angle: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastFtlJump {
    pub bypass_to: Box<Option<Number>>,
    pub jump_method: Box<Option<String>>,
    pub from: Box<Option<Position>>,
    pub bypass_from: Box<Option<Number>>,
    pub fleet: Box<Option<Number>>,
    pub to: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OrbitUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    OrbitClass(OrbitClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitClass {
    pub index: Box<Option<Number>>,
    pub orbitable: Box<Option<OrbitOrbitable>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitOrbitable {
    pub planet: Box<Option<Number>>,
    pub starbase: Box<Option<Number>>,
    pub megastructure: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovementManagerPath {
    pub date: Box<Option<String>>,
    pub node: Box<Option<NodeUnion>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum NodeUnion {
    NodeElement(NodeElement),
    NodeElementArray(Vec<NodeElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeElement {
    pub ftl: Box<Option<String>>,
    pub coordinate: Box<Option<Position>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetName {
    pub variables: Box<Option<Vec<HilariousVariable>>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HilariousVariable {
    pub value: Box<Option<PlanetName>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetName {
    pub key: Box<Option<String>>,
    pub literal: Box<Option<String>>,
    pub variables: Box<Option<Vec<AmbitiousVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmbitiousVariable {
    pub key: Box<Option<String>>,
    pub value: Box<Option<StickyValue>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StickyValue {
    pub key: Box<Option<String>>,
    pub literal: Box<Option<String>>,
    pub variables: Box<Option<Vec<CunningVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CunningVariable {
    pub value: Box<Option<SpeciesAdjectiveClass>>,
    pub key: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderClass {
    pub build_orbital_station_order: Box<Option<Vec<BuildOrbitalStationOrder>>>,
    pub survey_planet_order: Box<Option<SurveyPlanetOrder>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SurveyPlanetOrder {
    EtOrder(EtOrder),
    EtOrderArray(Vec<EtOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub can_change_composition: Box<Option<String>>,
    pub garrison: Box<Option<String>>,
    pub spawn_debris: Box<Option<String>>,
    pub is_boss: Box<Option<String>>,
    pub uses_naval_capacity: Box<Option<String>>,
    pub can_change_leader: Box<Option<String>>,
    pub can_upgrade: Box<Option<String>>,
    pub can_disband: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetStrikeCraft {
    pub crafts: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetVariables {
    pub giga_asteroid_ship_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FleetTemplateValue {
    FleetTemplateClass(FleetTemplateClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetTemplateClass {
    pub home_base: Box<Option<HomeBase>>,
    pub fleet_template_design: Box<Option<Vec<FleetTemplateDesign>>>,
    pub count: Box<Option<Number>>,
    pub all_queued: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub fleet_size: Box<Option<Number>>,
    pub fleet: Box<Option<Number>>,
    pub is_edited_by_human: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FleetTemplateDesign {
    pub design: Box<Option<Number>>,
    pub count: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HomeBase {
    pub orbitable: Box<Option<BuildableColonyShipOrbitable>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GalacticCommunity {
    pub proposed: Box<Option<Vec<Number>>>,
    pub members: Box<Option<Vec<Number>>>,
    pub days: Box<Option<Number>>,
    pub election: Box<Option<Number>>,
    pub last: Box<Option<Number>>,
    pub community_formed: Box<Option<String>>,
    pub envoy: Box<Option<Vec<Number>>>,
    pub could_end_senate_session: Box<Option<String>>,
    pub category_timers: Box<Option<CategoryTimers>>,
    pub passed: Box<Option<Vec<Number>>>,
    pub voting: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryTimers {
    pub resolution_category_wexternal: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GalacticObject {
    pub star_class: Box<Option<String>>,
    pub trade_hub: Box<Option<TradeHub>>,
    pub bypasses: Box<Option<Vec<Number>>>,
    pub hyperlane: Box<Option<Vec<Hyperlane>>>,
    pub has_access_to_relay_network: Box<Option<String>>,
    pub trade_collection: Box<Option<TradeCollectionUnion>>,
    pub sector: Box<Option<Number>>,
    pub colonies: Box<Option<Vec<Number>>>,
    pub planet: Box<Option<GalacticObjectUnion>>,
    pub starbases: Box<Option<Vec<Number>>>,
    pub initializer: Box<Option<String>>,
    pub inner_radius: Box<Option<Number>>,
    pub outer_radius: Box<Option<Number>>,
    pub name: Box<Option<PluralClass>>,
    pub previous_owner: Box<Option<Number>>,
    pub trade_piracy: Box<Option<TradePiracy>>,
    pub natural_wormholes: Box<Option<Vec<Number>>>,
    pub megastructures: Box<Option<Vec<Number>>>,
    #[serde(rename = "type")]
    pub galactic_object_type: Box<Option<String>>,
    pub fleet_presence: Box<Option<Vec<Number>>>,
    pub aura_presence: Box<Option<Vec<Number>>>,
    pub coordinate: Box<Option<Position>>,
    pub flags: Box<Option<VecOrMap<Number>>>,
    pub discovery: Box<Option<Vec<Number>>>,
    pub ambient_object: Box<Option<Vec<Number>>>,
    pub asteroid_belts: Box<Option<Vec<AsteroidBelt>>>,
    pub init_parent: Box<Option<Number>>,
    pub ftl_inhibitor_presence: Box<Option<Vec<Number>>>,
    pub timed_modifier: Box<Option<FederationProgressionTimedModifier>>,
    pub orbital_line: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsteroidBelt {
    pub inner_radius: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub asteroid_belt_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hyperlane {
    pub length: Box<Option<Number>>,
    pub to: Box<Option<Number>>,
    pub bridge: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TradeCollectionUnion {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    TradeCollectionClass(TradeCollectionClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeCollectionClass {
    pub targets: Box<Option<Vec<TargetElement>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetElement {
    pub distance: Box<Option<Number>>,
    pub target: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeHub {
    pub collected: Box<Option<Number>>,
    pub destination: Box<Option<Number>>,
    pub sources: Box<Option<Vec<Number>>>,
    pub collected_from: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradePiracy {
    pub throughput: Box<Option<Number>>,
    pub used: Box<Option<Number>>,
    pub active: Box<Option<Number>>,
    pub max: Box<Option<Number>>,
    pub total: Box<Option<Number>>,
    pub targets: Box<Option<Vec<TargetElement>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GalacticObjectVariables {
    pub current_giga_asteroid_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Galaxy {
    pub crises: Box<Option<Number>>,
    pub core_radius: Box<Option<Number>>,
    pub advanced_starts_near_player: Box<Option<String>>,
    pub logistic_ceiling: Box<Option<Number>>,
    pub random_marauder_empires: Box<Option<String>>,
    pub primitive: Box<Option<Number>>,
    pub caravaneers_enabled: Box<Option<String>>,
    pub difficulty: Box<Option<String>>,
    pub shape: Box<Option<String>>,
    pub num_guaranteed_colonies: Box<Option<Number>>,
    pub random_empires: Box<Option<String>>,
    pub num_hyperlanes: Box<Option<Number>>,
    pub num_fallen_empires: Box<Option<Number>>,
    pub template: Box<Option<String>>,
    pub name: Box<Option<String>>,
    pub victory_year: Box<Option<Number>>,
    pub growth_scale: Box<Option<Number>>,
    pub mid_game_start: Box<Option<Number>>,
    pub design: Box<Option<Vec<DesignElement>>>,
    pub num_advanced_empires: Box<Option<Number>>,
    pub aggressiveness: Box<Option<String>>,
    pub lgate_enabled: Box<Option<String>>,
    pub clustered: Box<Option<String>>,
    pub habitability: Box<Option<Number>>,
    pub xeno_compatibility_enabled: Box<Option<String>>,
    pub random_advanced_empires: Box<Option<String>>,
    pub crisis_type: Box<Option<String>>,
    pub scaling: Box<Option<String>>,
    pub random_fallen_empires: Box<Option<String>>,
    pub end_game_start: Box<Option<Number>>,
    pub num_empires: Box<Option<Number>>,
    pub num_marauder_empires: Box<Option<Number>>,
    pub num_wormhole_pairs: Box<Option<Number>>,
    pub technology: Box<Option<Number>>,
    pub player_locations: Box<Option<String>>,
    pub ironman: Box<Option<String>>,
    pub num_gateways: Box<Option<Number>>,
    pub difficulty_adjusted_ai_modifiers: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignElement {
    pub spawn_enabled: Box<Option<String>>,
    pub room: Box<Option<String>>,
    pub adjective: Box<Option<CountryAdjective>>,
    pub ethic: Box<Option<RequiredComponentUnion>>,
    pub civics: Box<Option<Vec<String>>>,
    pub planet_class: Box<Option<String>>,
    pub origin: Box<Option<String>>,
    pub city_graphical_culture: Box<Option<String>>,
    pub species: Box<Option<DesignSpecies>>,
    pub ruler: Box<Option<Ruler>>,
    pub initializer: Box<Option<String>>,
    pub advisor_voice_type: Box<Option<String>>,
    pub spawn_as_fallen: Box<Option<String>>,
    pub key: Box<Option<String>>,
    pub name: Box<Option<TentacledName>>,
    pub government: Box<Option<String>>,
    pub planet_name: Box<Option<PluralClass>>,
    pub authority: Box<Option<String>>,
    pub graphical_culture: Box<Option<String>>,
    pub ignore_portrait_duplication: Box<Option<String>>,
    pub ship_prefix: Box<Option<PluralClass>>,
    pub system_name: Box<Option<PluralClass>>,
    pub empire_flag: Box<Option<Flag>>,
    pub flags: Box<Option<Vec<String>>>,
    pub secondary_species: Box<Option<SecondarySpecies>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TentacledName {
    pub key: Box<Option<String>>,
    pub literal: Box<Option<String>>,
    pub variables: Box<Option<Vec<LastAllianceNameVariable>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ruler {
    pub name: Box<Option<FluffyName>>,
    pub texture: Box<Option<Number>>,
    pub portrait: Box<Option<String>>,
    pub attachment: Box<Option<Number>>,
    pub clothes: Box<Option<Number>>,
    pub gender: Box<Option<String>>,
    pub leader_class: Box<Option<String>>,
    #[serde(rename = "trait")]
    pub ruler_trait: Box<Option<RequiredComponentUnion>>,
    pub heir_title_female: Box<Option<PluralClass>>,
    pub ruler_title: Box<Option<PluralClass>>,
    pub heir_title: Box<Option<RulerTitleFemaleClass>>,
    pub ruler_title_female: Box<Option<RulerTitleFemaleClass>>,
    pub custom_biography: Box<Option<PluralClass>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecondarySpecies {
    pub species_name: Box<Option<RulerTitleFemaleClass>>,
    pub portrait: Box<Option<String>>,
    pub species_plural: Box<Option<RulerTitleFemaleClass>>,
    pub species_adjective: Box<Option<RulerTitleFemaleClass>>,
    pub name_list: Box<Option<String>>,
    pub gender: Box<Option<String>>,
    pub class: Box<Option<String>>,
    #[serde(rename = "trait")]
    pub secondary_species_trait: Box<Option<Vec<String>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignSpecies {
    pub portrait: Box<Option<String>>,
    pub species_adjective: Box<Option<SpeciesAdjectiveClass>>,
    pub gender: Box<Option<String>>,
    pub species_name: Box<Option<PluralClass>>,
    pub name_list: Box<Option<String>>,
    #[serde(rename = "trait")]
    pub species_trait: Box<Option<RequiredComponentUnion>>,
    pub class: Box<Option<String>>,
    pub species_plural: Box<Option<PluralClass>>,
    pub species_bio: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalShipDesign {
    pub name: Box<Option<RulerTitleFemaleClass>>,
    pub ship_design: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundCombat {
    #[serde(rename = "50331648")]
    pub the_50331648: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LeaderValue {
    LeaderLeader(LeaderLeader),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderLeader {
    pub traits: Box<Option<RequiredComponentUnion>>,
    pub country: Box<Option<Number>>,
    pub level: Box<Option<Number>>,
    pub ethic: Box<Option<String>>,
    pub design: Box<Option<LeaderDesign>>,
    pub name: Box<Option<LeaderName>>,
    pub experience: Box<Option<Number>>,
    pub age: Box<Option<Number>>,
    pub date: Box<Option<String>>,
    pub creator: Box<Option<Number>>,
    pub class: Box<Option<String>>,
    pub species: Box<Option<Number>>,
    pub planet: Box<Option<Number>>,
    pub portrait: Box<Option<String>>,
    pub recruitment_date: Box<Option<String>>,
    pub gender: Box<Option<String>>,
    pub location: Box<Option<Location>>,
    pub job: Box<Option<String>>,
    pub cooldown: Box<Option<Number>>,
    pub council_location: Box<Option<Location>>,
    pub can_see_in_list: Box<Option<String>>,
    pub can_manually_change_location: Box<Option<String>>,
    pub event_leader: Box<Option<String>>,
    pub flags: Box<Option<LeaderFlags>>,
    pub custom_description: Box<Option<String>>,
    pub immortal: Box<Option<String>>,
    pub date_added: Box<Option<String>>,
    pub external: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    #[serde(rename = "type")]
    pub location_type: Box<Option<String>>,
    pub assignment: Box<Option<String>>,
    pub area: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderFlags {
    pub leader_death_events_blocked: Box<Option<Number>>,
    pub fe_pouchkinn_leader: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderName {
    pub use_full_regnal_name: Box<Option<String>>,
    pub full_names: Box<Option<LastKilledCountryName>>,
    pub regnal_number: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderVariables {
    pub leader_pending_negative_traits_unmodified: Box<Option<Number>>,
    pub ruler_age_mod: Box<Option<Number>>,
    pub heir_age_mod: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Market {
    pub resources_sold: Box<Option<Resources>>,
    pub next_monthly_trade_item_id: Box<Option<Number>>,
    pub country: Box<Option<Number>>,
    pub id: Box<Option<Vec<Number>>>,
    pub resources_bought: Box<Option<Resources>>,
    pub internal_market_fluctuations: Box<Option<InternalMarketFluctuations>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalMarketFluctuations {
    pub country: Box<Option<Vec<Number>>>,
    pub resources: Box<Option<Vec<PopCategoryXenoWardElement>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resources {
    pub amount: Box<Option<Vec<Vec<Number>>>>,
    pub country: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Megastructure {
//     pub build_queue: Box<Option<Number>>,
//     pub planet: Box<Option<Number>>,
//     pub orbitals: Box<Option<VecOrStruct<Orbitals>>>,
//     #[serde(rename = "type")]
//     pub megastructure_type: Box<Option<String>>,
//     pub coordinate: Box<Option<Position>>,
//     pub owner: Box<Option<Number>>,
//     pub bypass: Box<Option<Number>>,
//     pub graphical_culture: Box<Option<String>>,
//     pub variables: Box<Option<VecOrStruct<MegastructureVariables>>>,
//     pub flags: Box<Option<VecOrStruct<MegastructureFlags>>>,
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Megastructure {
    #[serde(rename = "type")]
    pub megastructure_type: Option<String>,
    pub coordinate: Option<Position>,
    pub owner: Option<Number>,
    pub planet: Option<Number>,
    pub orbitals: Option<Orbitals>,
    pub build_queue: Option<Number>,
    pub bypass: Option<Number>,
    pub upgrade: Option<Upgrade>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimedModifiersClass {
    pub items: Option<Vec<TimedModifiersItem>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimedModifiersItem {
    pub modifier: Option<String>,
    pub days: Option<Number>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Upgrade {
    pub upgrade_to: Option<String>,
    pub progress: Option<f64>,
    pub halted: Option<i64>,
    pub indefinitely_halted: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MegastructureFlags {
    pub completed_mega: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Orbitals {
    Anything(VecOrMap<serde_json::Value>),
    IntegerMap(Vec<Number>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MegastructureVariables {
    pub giga_asteroid_megastructure_id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub technology: Box<Option<String>>,
    pub notification: Box<Option<Number>>,
    pub message_type: Box<Option<String>>,
    #[serde(rename = "type")]
    pub purple_type: Box<Option<String>>,
    pub coordinate: Box<Option<Position>>,
    pub localization: Box<Option<String>>,
    pub date: Box<Option<String>>,
    pub receiver: Box<Option<Number>>,
    pub end: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageVariable {
    pub key: Box<Option<String>>,
    pub value: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MissileValue {
    MissileMissile(MissileMissile),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissileMissile {
    pub speed: Box<Option<Number>>,
    pub armor: Box<Option<Number>>,
    pub forward_y: Box<Option<Number>>,
    pub retargets: Box<Option<Number>>,
    pub forward_x: Box<Option<Number>>,
    pub health: Box<Option<Number>>,
    pub rotation: Box<Option<Number>>,
    pub fleet: Box<Option<Number>>,
    pub target: Box<Option<Number>>,
    pub rotation_speed: Box<Option<Number>>,
    pub position: Box<Option<Position>>,
    pub owner: Box<Option<Number>>,
    pub shield: Box<Option<Number>>,
    pub weapon_component_template: Box<Option<String>>,
    pub rotation_rate: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NaturalWormhole {
    pub coordinate: Box<Option<Position>>,
    pub bypass: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nebula {
    pub name: Box<Option<RulerTitleFemaleClass>>,
    pub radius: Box<Option<Number>>,
    pub galactic_object: Box<Option<GalacticObjectUnion>>,
    pub coordinate: Box<Option<Position>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenPlayerEventSelectionHistory {
    pub selected: Box<Option<Vec<Selected>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Selected {
    pub player_event: Box<Option<Number>>,
    pub human: Box<Option<Number>>,
    pub option: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrbitalLine {
    pub coordinate: Box<Option<Position>>,
    pub moon_of: Box<Option<Number>>,
    pub orbit: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planets {
    pub planet: Box<Option<VecOrMap<PlanetValue>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PlanetValue {
    PlanetClass(PlanetClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetClass {
    pub bombardment_damage: Box<Option<Number>>,
    pub automated_development: Box<Option<String>>,
    pub controller: Box<Option<Number>>,
    pub auto_slots_taken: Box<Option<Vec<String>>>,
    pub free_housing: Box<Option<Number>>,
    pub planet_orbitals: Box<Option<Orbitals>>,
    pub amenities_usage: Box<Option<Number>>,
    pub army_build_queue: Box<Option<Number>>,
    pub stability: Box<Option<Number>>,
    pub free_amenities: Box<Option<Number>>,
    pub last_bombardment: Box<Option<String>>,
    pub migration: Box<Option<Number>>,
    pub flags: Box<Option<PlanetFlags>>,
    pub surveyed_by: Box<Option<Number>>,
    pub crime: Box<Option<Number>>,
    pub amenities: Box<Option<Number>>,
    pub ascension_tier: Box<Option<Number>>,
    pub favorite_jobs: Box<Option<Vec<Box<Option<serde_json::Value>>>>>,
    pub planet_size: Box<Option<Number>>,
    pub total_housing: Box<Option<Number>>,
    pub num_sapient_pops: Box<Option<Number>>,
    pub coordinate: Box<Option<Position>>,
    pub name: Box<Option<PlanetName>>,
    pub kill_pop: Box<Option<String>>,
    pub entity: Box<Option<Number>>,
    pub recalc_pops: Box<Option<String>>,
    pub orbit: Box<Option<Number>>,
    pub housing_usage: Box<Option<Number>>,
    pub planet_class: Box<Option<String>>,
    pub employable_pops: Box<Option<Number>>,
    pub build_queue: Box<Option<Number>>,
    pub manual_designation_changed_date: Box<Option<String>>,
    pub orbital_defence: Box<Option<Number>>,
    pub buildings: Box<Option<Vec<Number>>>,
    pub jobs_cache: Box<Option<Vec<JobsCache>>>,
    pub species_refs: Box<Option<Vec<Number>>>,
    pub species_information: Box<Option<VecOrMap<SpeciesInformation>>>,
    pub deposits: Box<Option<Vec<Number>>>,
    pub district: Box<Option<Vec<String>>>,
    pub final_designation: Box<Option<String>>,
    pub planet_automation_settings: Box<Option<Vec<String>>>,
    pub timed_modifier: Box<Option<CountryTimedModifier>>,
    pub colonize_date: Box<Option<String>>,
    pub owner: Box<Option<Number>>,
    pub growth: Box<Option<Number>>,
    pub original_owner: Box<Option<Number>>,
    pub pop_to_kill_from_devastation: Box<Option<Number>>,
    pub pop: Box<Option<Vec<Number>>>,
    pub prevent_anomaly: Box<Option<String>>,
    pub army: Box<Option<Vec<Number>>>,
    pub growth_species_ref: Box<Option<Number>>,
    pub governor: Box<Option<Number>>,
    pub trigger_megastructure_icon: Box<Option<String>>,
    pub shipclass_orbital_station: Box<Option<Number>>,
    pub has_ring: Box<Option<String>>,
    pub job_priority: Box<Option<JobPriorityUnion>>,
    pub designation: Box<Option<String>>,
    pub moons: Box<Option<Vec<Number>>>,
    pub last_building_changed: Box<Option<String>>,
    pub last_district_changed: Box<Option<String>>,
    pub moon_of: Box<Option<Number>>,
    pub is_moon: Box<Option<String>>,
    pub planet_modifier: Box<Option<RequiredComponentUnion>>,
    pub pop_assembly: Box<Option<Number>>,
    pub assembling_species_ref: Box<Option<Number>>,
    pub entity_name: Box<Option<String>>,
    pub explicit_entity: Box<Option<String>>,
    pub externally_owned_buildings: Box<Option<Vec<ExternallyOwnedBuilding>>>,
    pub externally_owned_build_queues: Box<Option<Vec<ExternallyOwnedBuildQueue>>>,
    pub delayed_event: Box<Option<DelayedEventUnion>>,
    pub enslaved_species_refs: Box<Option<Vec<Number>>>,
    pub forced_growth_species_ref: Box<Option<Number>>,
    pub planet_class_changed: Box<Option<String>>,
    pub anomaly: Box<Option<String>>,
    pub surveyed: Box<Option<String>>,
    pub is_under_colonization: Box<Option<String>>,
    pub entity_planet_class: Box<Option<String>>,
    pub picture: Box<Option<String>>,
    pub atmosphere_color: Box<Option<Vec<Number>>>,
    pub atmosphere_width: Box<Option<Number>>,
    pub atmosphere_intensity: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternallyOwnedBuildQueue {
    pub value: Box<Option<Number>>,
    pub key: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternallyOwnedBuilding {
    pub buildings: Box<Option<Vec<Number>>>,
    pub building_owner: Box<Option<Number>>,
    pub owner_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetFlags {
    pub cant_build_here: Box<Option<Number>>,
    pub megastructure: Box<Option<Number>>,
    pub oe_player_capital: Box<Option<Number>>,
    pub player_oe_planet: Box<Option<Number>>,
    pub starting_deposit: Box<Option<Number>>,
    pub starting_mining_target: Box<Option<Number>>,
    pub starting_research_target: Box<Option<Number>>,
    pub drow_valshaquellar_project: Box<Option<Number>>,
    pub habitat: Box<Option<Number>>,
    pub drow_valshaquellar_start: Box<Option<Number>>,
    pub tulshar_capital: Box<Option<Number>>,
    pub has_megastructure: Box<Option<Number>>,
    pub ignore_startup_effect: Box<Option<Number>>,
    pub tasty_desert: Box<Option<Number>>,
    pub toxic_terraforming_candidate: Box<Option<Number>>,
    pub ancient_history_planet: Box<Option<Number>>,
    pub has_giga_wrecked_ship: Box<Option<Number>>,
    pub abandoned_terraforming_planet: Box<Option<Number>>,
    pub forbid_guillis_planet_modifiers: Box<Option<Number>>,
    pub upl_ongoing_planet: Box<Option<Number>>,
    pub pre_sapient_planet: Box<Option<Number>>,
    pub superior_colonies_can_be_boosted: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub prescripted_ideal: Box<Option<Number>>,
    #[serde(rename = "3_year_owner_change_flag")]
    pub the_3_year_owner_change_flag: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub colony_event: Box<Option<Number>>,
    pub can_have_precursor_event: Box<Option<Number>>,
    pub pre_ftl_hive_mind: Box<Option<Number>>,
    pub pre_ftl_default: Box<Option<Number>>,
    pub has_giga_asteroid_id: Box<Option<Number>>,
    pub has_fe_asteroid_artillery: Box<Option<Number>>,
    pub asteroid_has_artillery: Box<Option<Number>>,
    pub fallen_empire_world: Box<Option<Number>>,
    pub cant_target_this_planet: Box<Option<Number>>,
    pub fallen_shield_world: Box<Option<Number>>,
    pub dormant_war_planet: Box<Option<Number>>,
    pub dormant_war_moon: Box<Option<Number>>,
    pub holy_world_1: Box<Option<Number>>,
    pub holy_world_2: Box<Option<Number>>,
    pub holy_world_3: Box<Option<Number>>,
    pub holy_world_4: Box<Option<Number>>,
    pub fe_the_preserve: Box<Option<Number>>,
    pub czyrni_planet: Box<Option<Number>>,
    pub pyorun_planet: Box<Option<Number>>,
    pub crisis_vital_planet: Box<Option<Number>>,
    pub machine_world_3: Box<Option<Number>>,
    pub raid_source: Box<Option<Number>>,
    pub polaris_patrol_1: Box<Option<Number>>,
    pub polaris_patrol_2: Box<Option<Number>>,
    pub polaris_patrol_3: Box<Option<Number>>,
    pub machine_world_4: Box<Option<Number>>,
    pub ratling_planet: Box<Option<Number>>,
    pub ruinous_core_planet: Box<Option<Number>>,
    pub lost_encampment_planet: Box<Option<Number>>,
    pub silent_colony_planet: Box<Option<Number>>,
    pub decayed_hub_planet: Box<Option<Number>>,
    pub crumbling_borough_planet: Box<Option<Number>>,
    pub fallen_outpost_planet: Box<Option<Number>>,
    pub living_planet: Box<Option<Number>>,
    pub hot_zone: Box<Option<Number>>,
    pub frozen_in_time_flag: Box<Option<Number>>,
    pub machine_world_2: Box<Option<Number>>,
    pub big_rip_planet: Box<Option<Number>>,
    pub machine_world_1: Box<Option<Number>>,
    pub tiyanki_giant1: Box<Option<Number>>,
    pub tiyanki_giant2: Box<Option<Number>>,
    pub tiyanki_giant3: Box<Option<Number>>,
    pub tiyanki_giant4: Box<Option<Number>>,
    pub graveyard_star: Box<Option<Number>>,
    pub graveyard_site: Box<Option<Number>>,
    pub graveyard_asteroid: Box<Option<Number>>,
    pub graveyard_gas: Box<Option<Number>>,
    pub phaseshifting_active: Box<Option<Number>>,
    pub primitive_robot_planet: Box<Option<Number>>,
    pub unrest_timer: Box<Option<AsteroidSightedPreFtlGlobal>>,
    pub time_loop_world: Box<Option<Number>>,
    pub is_time_loop_world: Box<Option<Number>>,
    pub giga_planet_mega: Box<Option<Number>>,
    pub salvager_enclave_planet: Box<Option<Number>>,
    pub guardians_traders_planet: Box<Option<Number>>,
    pub outergate_star: Box<Option<Number>>,
    pub guardians_artists_planet: Box<Option<Number>>,
    pub hatchling_egg: Box<Option<Number>>,
    pub hatchling_will_trigger: Box<Option<Number>>,
    pub shroudwalker_enclave_planet: Box<Option<Number>>,
    pub amoeba_star: Box<Option<Number>>,
    pub amoeba_giant1: Box<Option<Number>>,
    pub amoeba_ice_asteroid_1: Box<Option<Number>>,
    pub amoeba_ice_asteroid_2: Box<Option<Number>>,
    pub amoeba_ice_asteroid_3: Box<Option<Number>>,
    pub patrol_1: Box<Option<Number>>,
    pub patrol_2: Box<Option<Number>>,
    pub patrol_3: Box<Option<Number>>,
    pub patrol_4: Box<Option<Number>>,
    pub patrol_5: Box<Option<Number>>,
    pub patrol_6: Box<Option<Number>>,
    pub patrol_7: Box<Option<Number>>,
    pub patrol_8: Box<Option<Number>>,
    pub pre_ftl_life_seeded: Box<Option<Number>>,
    pub hiver_asteroid_1: Box<Option<Number>>,
    pub hiver_asteroid_2: Box<Option<Number>>,
    pub hiver_asteroid_3: Box<Option<Number>>,
    pub hiver_asteroid_4: Box<Option<Number>>,
    pub hiver_asteroid_5: Box<Option<Number>>,
    pub ruined_orbital_ring_planet: Box<Option<Number>>,
    pub minigalaxy_planet: Box<Option<Number>>,
    pub minigalaxy_moon: Box<Option<Number>>,
    pub guardians_wraith_pulsar: Box<Option<Number>>,
    pub lone_defender_star: Box<Option<Number>>,
    pub lone_defender_planet: Box<Option<Number>>,
    pub lone_defender_moon: Box<Option<Number>>,
    pub hillos_patrol_1: Box<Option<Number>>,
    pub hillos_patrol_2: Box<Option<Number>>,
    pub hillos_patrol_3: Box<Option<Number>>,
    pub hillos_patrol_4: Box<Option<Number>>,
    pub pre_ftl_subterranean: Box<Option<Number>>,
    pub broken_fortress_1: Box<Option<Number>>,
    pub pre_ftl_mechanists: Box<Option<Number>>,
    pub omnicodex_planet: Box<Option<Number>>,
    pub research_habitat: Box<Option<Number>>,
    pub pre_ftl_void_dwellers: Box<Option<Number>>,
    pub ore_grinder_planet: Box<Option<Number>>,
    pub orbital_arcology: Box<Option<Number>>,
    pub giga_planetary_computer: Box<Option<Number>>,
    pub giga_rogue_ai_planetary_computer: Box<Option<Number>>,
    pub orbital_arcology_1: Box<Option<Number>>,
    pub planet_earth: Box<Option<Number>>,
    pub giga_luna: Box<Option<Number>>,
    pub planet_mars: Box<Option<Number>>,
    pub advanced_habitat_2: Box<Option<Number>>,
    pub advanced_habitat: Box<Option<Number>>,
    pub mining_habitat: Box<Option<Number>>,
    pub energy_habitat: Box<Option<Number>>,
    pub random_asteroid_name_20: Box<Option<Number>>,
    pub random_asteroid_name_3: Box<Option<Number>>,
    pub random_asteroid_name_15: Box<Option<Number>>,
    pub random_asteroid_name_6: Box<Option<Number>>,
    pub random_asteroid_name_7: Box<Option<Number>>,
    pub random_asteroid_name_12: Box<Option<Number>>,
    pub random_asteroid_name_9: Box<Option<Number>>,
    pub random_asteroid_name_24: Box<Option<Number>>,
    pub jublio: Box<Option<Number>>,
    pub veene: Box<Option<Number>>,
    pub doughland: Box<Option<Number>>,
    pub gigaflusion: Box<Option<Number>>,
    pub kaiser_update: Box<Option<Number>>,
    pub xenon: Box<Option<Number>>,
    pub gigamuno: Box<Option<Number>>,
    pub merries: Box<Option<Number>>,
    pub ledigen: Box<Option<Number>>,
    pub glaka: Box<Option<Number>>,
    pub nesar: Box<Option<Number>>,
    pub creeo: Box<Option<Number>>,
    pub mlekane: Box<Option<Number>>,
    pub jupitwo: Box<Option<Number>>,
    pub corrona: Box<Option<Number>>,
    pub jupitwo1: Box<Option<Number>>,
    pub jupitwo2: Box<Option<Number>>,
    pub jupitwo3: Box<Option<Number>>,
    pub giga_dirt_behemoth: Box<Option<Number>>,
    pub giga_eaw_planet: Box<Option<Number>>,
    pub giga_eaw_moon: Box<Option<Number>>,
    pub giga_eaw_sun: Box<Option<Number>>,
    pub giga_dirt_planet_1: Box<Option<Number>>,
    pub giga_dirt_planet_2: Box<Option<Number>>,
    pub giga_dirt_planet_3: Box<Option<Number>>,
    pub giga_dirt_planet_4: Box<Option<Number>>,
    pub giga_dirt_planet_5: Box<Option<Number>>,
    pub giga_eeloo: Box<Option<Number>>,
    pub giga_terraforming_megastructure: Box<Option<Number>>,
    pub katzenland: Box<Option<Number>>,
    pub paluushia: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum JobPriorityUnion {
    JobPriorityElement(JobPriorityElement),
    JobPriorityElementArray(Vec<JobPriorityElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobPriorityElement {
    pub priority: Box<Option<Number>>,
    pub job: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobsCache {
    pub max_without_prio: Box<Option<Number>>,
    pub num_employed: Box<Option<Number>>,
    pub max_employed: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeciesInformation {
    pub num_pops: Box<Option<Number>>,
    pub num_enslaved: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: Box<Option<String>>,
    pub country: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pop {
    pub crime: Box<Option<Number>>,
    pub has_random_ethics: Box<Option<String>>,
    pub species: Box<Option<Number>>,
    pub housing_usage: Box<Option<Number>>,
    pub enslaved: Box<Option<String>>,
    pub job: Box<Option<String>>,
    pub can_vote: Box<Option<String>>,
    pub can_fill_worker_job: Box<Option<String>>,
    pub can_fill_specialist_job: Box<Option<String>>,
    pub planet: Box<Option<Number>>,
    pub force_faction_evaluation: Box<Option<String>>,
    pub can_fill_ruler_job: Box<Option<String>>,
    pub happiness: Box<Option<Number>>,
    pub can_fill_drone_job: Box<Option<String>>,
    pub can_migrate: Box<Option<String>>,
    pub amenities_usage: Box<Option<Number>>,
    pub category: Box<Option<String>>,
    pub power: Box<Option<Number>>,
    pub job_weights_cache: Box<Option<Vec<Number>>>,
    pub diplomatic_weight: Box<Option<Number>>,
    pub spawned_armies: Box<Option<Vec<Number>>>,
    pub promotion_date: Box<Option<String>>,
    pub ethos: Box<Option<PopEthos>>,
    pub demotion_time: Box<Option<String>>,
    pub demotion: Box<Option<String>>,
    pub former_job: Box<Option<String>>,
    pub flags: Box<Option<PopFlags>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopEthos {
    pub ethic: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PopFlags {
    pub organic_fallen_empire_pop: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RandomNameDatabase {
    pub asteroid_prefix: Box<Option<Vec<String>>>,
    pub nebula_names: Box<Option<Vec<String>>>,
    pub species_modification_prefix: Box<Option<Vec<String>>>,
    pub species_modification_postfix: Box<Option<Vec<String>>>,
    pub asteroid_postfix: Box<Option<Vec<Vec<String>>>>,
    pub star_names: Box<Option<Vec<String>>>,
    pub black_hole_names: Box<Option<Vec<String>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resolution {
    pub opponents: Box<Option<Vec<Number>>>,
    pub country: Box<Option<Number>>,
    pub supporters: Box<Option<Vec<Number>>>,
    pub voting: Box<Option<VecOrMap<String>>>,
    #[serde(rename = "type")]
    pub resolution_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedLeaders {
    pub trapped_in_storm: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sector {
    pub auto_expand: Box<Option<String>>,
    #[serde(rename = "type")]
    pub sector_type: Box<Option<String>>,
    pub systems: Box<Option<Vec<Number>>>,
    pub name: Box<Option<SectorName>>,
    pub owner: Box<Option<Number>>,
    pub resources: Box<Option<Number>>,
    pub local_capital: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ShipDesignValue {
    ShipDesignClass(ShipDesignClass),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipDesignClass {
    pub ship_size: Box<Option<String>>,
    pub name: Box<Option<RulerTitleFemaleClass>>,
    pub section: Box<Option<ShipDesignSection>>,
    pub auto_gen_design: Box<Option<String>>,
    pub upgrade_ship_components: Box<Option<String>>,
    pub custom_design: Box<Option<String>>,
    pub required_component: Box<Option<RequiredComponentUnion>>,
    pub initial_design: Box<Option<String>>,
    pub allow_buildable_trigger: Box<Option<String>>,
    pub is_special_buildable: Box<Option<String>>,
    pub is_event_design: Box<Option<String>>,
    pub country_type: Box<Option<String>>,
    pub ship_owner_type: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ShipDesignSection {
    FluffySection(FluffySection),
    PurpleSectionArray(Vec<PurpleSection>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleSection {
    pub template: Box<Option<String>>,
    pub slot: Box<Option<String>>,
    pub component: Box<Option<ComponentUnion>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ComponentUnion {
    ComponentElement(ComponentElement),
    ComponentElementArray(Vec<ComponentElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentElement {
    pub slot: Box<Option<String>>,
    pub template: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffySection {
    pub template: Box<Option<String>>,
    pub slot: Box<Option<String>>,
    pub component: Box<Option<Vec<ComponentElement>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ship {
    pub post_move_angle: Box<Option<Number>>,
    pub reserve: Box<Option<Number>>,
    pub target_coordinate: Box<Option<Position>>,
    pub created_this_update: Box<Option<String>>,
    pub hitpoints: Box<Option<Number>>,
    pub forward_y: Box<Option<Number>>,
    pub shield_hitpoints: Box<Option<Number>>,
    pub section: Box<Option<ShipSection>>,
    pub max_hitpoints: Box<Option<Number>>,
    pub name: Box<Option<FleetName>>,
    pub ship_design: Box<Option<Number>>,
    pub max_armor_hitpoints: Box<Option<Number>>,
    pub rotation: Box<Option<Number>>,
    pub graphical_culture: Box<Option<String>>,
    pub forward_x: Box<Option<Number>>,
    pub upgrade_progress: Box<Option<Number>>,
    pub leader: Box<Option<Number>>,
    pub design_upgrade: Box<Option<Number>>,
    pub coordinate: Box<Option<Position>>,
    pub max_shield_hitpoints: Box<Option<Number>>,
    pub disengagement_opportunities: Box<Option<Number>>,
    pub armor_hitpoints: Box<Option<Number>>,
    pub combat_action: Box<Option<Number>>,
    pub disengagement_opportunities_used: Box<Option<Number>>,
    pub fleet: Box<Option<Number>>,
    pub cloaking_animation_progress: Box<Option<Number>>,
    pub timed_modifier: Box<Option<FederationProgressionTimedModifier>>,
    pub disable_at_health: Box<Option<Number>>,
    pub next_weapon_index: Box<Option<Number>>,
    pub enable_at_health: Box<Option<Number>>,
    pub speed: Box<Option<Number>>,
    pub formation_pos: Box<Option<FormationPos>>,
    pub flags: Box<Option<ShipFlags>>,
    pub targeting: Box<Option<Number>>,
    pub kill_target: Box<Option<Number>>,
    pub last_damage: Box<Option<String>>,
    pub experience: Box<Option<Number>>,
    pub auras: Box<Option<Vec<Aura>>>,
    pub aura_modifier: Box<Option<AuraModifier>>,
    pub army: Box<Option<Number>>,
    pub upgradable: Box<Option<String>>,
    pub disabled_by_event: Box<Option<String>>,
    pub is_being_repaired: Box<Option<String>>,
    pub shield_recharge_time: Box<Option<Number>>,
    pub ship_modifier: Box<Option<ShipModifier>>,
    pub homepop: Box<Option<Homepop>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuraModifier {
    pub ship_fire_rate_mult: Box<Option<Number>>,
    pub ship_tracking_add: Box<Option<Number>>,
    pub ship_speed_mult: Box<Option<Number>>,
    pub ship_shield_mult: Box<Option<Number>>,
    pub ship_ftl_jumpdrive_range_mult: Box<Option<Number>>,
    pub ship_jumpdrive_cooldown_mult: Box<Option<Number>>,
    pub ship_weapon_damage: Box<Option<Number>>,
    pub ship_weapon_range_mult: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aura {
    pub id: Box<Option<Id>>,
    pub source: Box<Option<Source>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id {
    pub priority: Box<Option<Number>>,
    pub id: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    pub ship: Box<Option<Number>>,
    pub aura: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipFlags {
    pub speedchange_incombat: Box<Option<Number>>,
    pub fortress_vault: Box<Option<Number>>,
    pub lone_defender_ship: Box<Option<Number>>,
    pub cara_home_tradestation: Box<Option<Number>>,
    pub here_be_the_dragon_ship: Box<Option<Number>>,
    pub giga_moon_fe: Box<Option<Number>>,
    pub giga_planet_fe: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Homepop {
    pub species: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ShipSection {
    StickySection(StickySection),
    TentacledSectionArray(Vec<TentacledSection>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TentacledSection {
    pub design: Box<Option<String>>,
    pub slot: Box<Option<String>>,
    pub weapon: Box<Option<StickyWeapon>>,
    pub strike_craft: Box<Option<SectionStrikeCraft>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SectionStrikeCraft {
    StrikeCraftElement(StrikeCraftElement),
    StrikeCraftElementArray(Vec<StrikeCraftElement>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrikeCraftElement {
    pub template: Box<Option<String>>,
    pub launch_time: Box<Option<Number>>,
    pub component_slot: Box<Option<String>>,
    pub count: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StickyWeapon {
    PurpleWeapon(PurpleWeapon),
    PurpleWeaponArray(Vec<PurpleWeapon>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleWeapon {
    pub index: Box<Option<Number>>,
    pub component_slot: Box<Option<String>>,
    pub template: Box<Option<String>>,
    pub cooldown: Box<Option<Number>>,
    pub shots_fired: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StickySection {
    pub design: Box<Option<String>>,
    pub slot: Box<Option<String>>,
    pub weapon: Box<Option<IndigoWeapon>>,
    pub strike_craft: Box<Option<SectionStrikeCraft>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IndigoWeapon {
    FluffyWeaponArray(Vec<FluffyWeapon>),
    TentacledWeapon(TentacledWeapon),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FluffyWeapon {
    pub template: Box<Option<String>>,
    pub cooldown: Box<Option<Number>>,
    pub component_slot: Box<Option<String>>,
    pub index: Box<Option<Number>>,
    pub target: Box<Option<Owner>>,
    pub shots_fired: Box<Option<Number>>,
    pub windup: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TentacledWeapon {
    pub component_slot: Box<Option<String>>,
    pub index: Box<Option<Number>>,
    pub template: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipModifier {
    pub ship_starting_experience_add: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Situations {
    pub situations: Box<Option<VecOrMap<SituationValue>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SituationValue {
    SituationClass(SituationClass),
    Anything(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SituationClass {
    pub target: Box<Option<SituationTarget>>,
    pub last_month_progress: Box<Option<Number>>,
    pub approach: Box<Option<String>>,
    pub flags: Box<Option<serde_json::Value>>,
    pub country: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub situation_type: Box<Option<String>>,
    pub progress: Box<Option<Number>>,
    pub stage_flags: Box<Option<Vec<String>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SituationTarget {
    #[serde(rename = "type")]
    pub target_type: Box<Option<String>>,
    pub id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeciesDb {
    pub gender: Box<Option<String>>,
    pub traits: Box<Option<SpeciesDbTraits>>,
    pub portrait: Box<Option<String>>,
    pub name_list: Box<Option<String>>,
    pub home_planet: Box<Option<Number>>,
    pub adjective: Box<Option<SpeciesAdjectiveClass>>,
    pub name: Box<Option<PluralClass>>,
    pub class: Box<Option<String>>,
    pub plural: Box<Option<PluralClass>>,
    pub base_ref: Box<Option<Number>>,
    pub name_data: Box<Option<String>>,
    pub flags: Box<Option<VecOrMap<Number>>>,
    pub sapient: Box<Option<String>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SpeciesDbTraits {
    AnythingArray(Vec<Box<Option<serde_json::Value>>>),
    TraitsClass(TraitsClass),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraitsClass {
    #[serde(rename = "trait")]
    pub traits_trait: Box<Option<RequiredComponentUnion>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpyNetwork {
    pub power: Box<Option<Number>>,
    pub formed: Box<Option<String>>,
    pub leader: Box<Option<Number>>,
    pub target: Box<Option<Number>>,
    pub owner: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StarbaseMgr {
    pub starbases: Box<Option<VecOrMap<Starbase>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Starbase {
    pub update_flag: Box<Option<Number>>,
    pub shipyard_build_queue: Box<Option<Number>>,
    pub ship_design: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub starbase_type: Box<Option<String>>,
    pub modules: Box<Option<VecOrMap<String>>>,
    pub build_queue: Box<Option<Number>>,
    pub orbitals: Box<Option<Orbitals>>,
    pub level: Box<Option<String>>,
    pub buildings: Box<Option<VecOrMap<String>>>,
    pub station: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StrikeCraftValue {
    PurpleStrikeCraft(PurpleStrikeCraft),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurpleStrikeCraft {
    pub target: Box<Option<Owner>>,
    pub health: Box<Option<Number>>,
    pub position: Box<Option<Position>>,
    pub forward_x: Box<Option<Number>>,
    pub component_slot: Box<Option<String>>,
    pub shield: Box<Option<Number>>,
    pub cached_max_speed: Box<Option<Number>>,
    pub owner: Box<Option<Number>>,
    pub cooldown: Box<Option<Number>>,
    pub speed: Box<Option<Number>>,
    pub action: Box<Option<Number>>,
    pub rotation_speed: Box<Option<Number>>,
    pub section: Box<Option<String>>,
    pub forward_y: Box<Option<Number>>,
    pub strike_craft_component_template: Box<Option<String>>,
    pub armor: Box<Option<Number>>,
    pub rotation: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInitializerCounter {
    pub count: Box<Option<Vec<Number>>>,
    pub initializer: Box<Option<Vec<String>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeRoute {
    pub delivered: Box<Option<Number>>,
    pub to: Box<Option<Number>>,
    pub from: Box<Option<Number>>,
    #[serde(rename = "type")]
    pub trade_route_type: Box<Option<String>>,
    pub path: Box<Option<Vec<PathElement>>>,
    pub owner: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathElement {
    pub delivered: Box<Option<Number>>,
    pub collected: Box<Option<Number>>,
    pub id: Box<Option<Number>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeRoutesManager {
    pub trade_routes: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsedSpecies {
    pub class: Box<Option<String>>,
    pub values: Box<Option<Vec<Number>>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct War {
    pub name: Option<WarName>,
    pub start_date: Option<String>,
    pub attackers: Option<Vec<Attacker>>,
    pub defenders: Option<Vec<Attacker>>,
    pub battles: Option<Vec<Battle>>,
    pub attacker_war_goal: Option<ErWarGoal>,
    pub defender_war_goal: Option<ErWarGoal>,
    pub have_defender_war_goal: Option<String>,
    pub attacker_war_exhaustion: Option<f64>,
    pub defender_war_exhaustion: Option<f64>,
    pub attacker_force_peace: Option<String>,
    pub attacker_force_peace_date: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErWarGoal {
    #[serde(rename = "type")]
    pub er_war_goal_type: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attacker {
    pub call_type: Option<String>,
    pub country: Option<Number>,
    pub caller: Option<Number>,
    pub fleets_gone_mia: Option<Number>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Battle {
    pub defenders: Option<Vec<Number>>,
    pub attackers: Option<Vec<Number>>,
    pub system: Option<Number>,
    pub planet: Option<Number>,
    pub attacker_war_exhaustion: Option<f64>,
    pub defender_war_exhaustion: Option<f64>,
    pub attacker_victory: Option<String>,
    pub date: Option<String>,
    pub attacker_losses: Option<Number>,
    pub defender_losses: Option<Number>,
    #[serde(rename = "type")]
    pub battle_type: Option<String>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarName {
    pub key: Option<String>,
    pub variables: Option<Vec<CunningVariable>>,
    #[serde(default)]
    _ignore: Option<serde_json::Value>,
}
