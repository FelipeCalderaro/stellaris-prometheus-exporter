use once_cell::sync::Lazy;

use prometheus::{GaugeVec, IntCounter, IntGaugeVec, Opts, Registry};

pub static REGISTRY: Lazy<Registry> = Lazy::new(|| Registry::new());
// -----------
pub static STELLARIS_INCOMING_REQUESTS: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("stellaris_incoming_requests", "Incoming Requests")
        .expect("metric can be created")
});

pub static STELLARIS_COUNTRY_POWER: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "stellaris_country_power",
            "The amount of specific Power Type for each country. (Economic, Military, Tech)",
        ),
        &["save_name", "power_type", "country"],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_MEGASTRUCTURES: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "stellaris_megastructures",
            "The amount of each megastrucuture in the game",
        ),
        &["save_name", "name", "owner"],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_COUNTRY_BALANCE: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "stellaris_country_balance",
            "The amount of each resource in the country silo. (Energy, Alloys, Minerals, etc...)",
        ),
        &["save_name", "country", "resource_name"],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_COUNTRY_VICTORY_STATUS: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "stellaris_country_victory_status",
            "The Victory rank or score for each Country",
        ),
        &["save_name", "country", "type"],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_COUNTRY_BATTLE_LOSSES: Lazy<GaugeVec> = Lazy::new(|| {
    GaugeVec::new(
        Opts::new(
            "stellaris_country_battle_losses",
            "The amount of losses for each battle between countries (a.k.a: War)",
        ),
        &[
            "save_name",
            "attacker",
            "defender",
            "battle_type",
            "loss",
            "id",
        ],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_COUNTRY_WAR_BATLLES: Lazy<IntGaugeVec> = Lazy::new(|| {
    IntGaugeVec::new(
        Opts::new(
            "stellaris_country_war_battles",
            "The amount of battles for each war between countries",
        ),
        &[
            "save_name",
            "attacker",
            "defender",
            "attacker_war_goal",
            "defender_war_goal",
            "war_name",
            "start_date",
            "id",
        ],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_COUNTRY_FLEETS: Lazy<IntGaugeVec> = Lazy::new(|| {
    IntGaugeVec::new(
        Opts::new(
            "stellaris_country_fleets",
            "The amount of fleets in each country.",
        ),
        &["save_name", "country"],
    )
    .expect("Could'nt create gauge")
});

pub static STELLARIS_COUNTRY_WAR_ALLIES: Lazy<IntGaugeVec> = Lazy::new(|| {
    IntGaugeVec::new(
        Opts::new(
            "stellaris_country_war_allies",
            "The number of allies for each country in case of war.",
        ),
        &["save_name", "country"],
    )
    .expect("Could'nt create gauge")
});

pub fn register_metrics() {
    REGISTRY
        .register(Box::new(STELLARIS_INCOMING_REQUESTS.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_COUNTRY_POWER.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_COUNTRY_BALANCE.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_COUNTRY_FLEETS.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_COUNTRY_VICTORY_STATUS.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_COUNTRY_WAR_ALLIES.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_MEGASTRUCTURES.clone()))
        .expect("Collector registered");
    REGISTRY
        .register(Box::new(STELLARIS_COUNTRY_WAR_BATLLES.clone()))
        .expect("Collector registered");
}
