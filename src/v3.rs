use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct KtxstatsV3 {
    pub version: i32,
    pub date: DateTime<Utc>,
    pub map: String,
    pub hostname: String,
    pub ip: String,
    pub port: i32,
    pub mode: String,
    pub tl: i32,
    pub dm: i32,
    pub tp: i32,
    pub duration: i32,
    pub demo: String,
    pub teams: Vec<String>,
    pub players: Vec<Player>,
}

impl TryFrom<&str> for KtxstatsV3 {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Player {
    pub top_color: i32,
    pub bottom_color: i32,
    pub ping: i32,
    pub login: String,
    pub name: String,
    pub team: String,
    pub stats: PlayerStats,
    pub dmg: PlayerDamage,
    pub xfer_rl: i32,
    pub xfer_lg: i32,
    pub spree: FragSpree,
    pub control: f64,
    pub speed: PlayerSpeed,
    pub weapons: Weapons,
    pub items: Items,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerSpeed {
    pub avg: f64,
    pub max: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PlayerDamage {
    pub taken: i32,
    pub given: i32,
    pub team: i32,
    #[serde(rename = "self")]
    pub dmg_self: i32,
    pub team_weapons: i32,
    pub enemy_weapons: i32,
    pub taken_to_die: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Items {
    pub health_15: Health,
    pub health_25: Health,
    pub health_100: Health,
    pub ga: Armor,
    pub ya: Armor,
    pub ra: Armor,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Armor {
    pub took: i32,
    pub time: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Health {
    pub took: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FragSpree {
    pub max: i32,
    pub quad: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PlayerStats {
    pub frags: i32,
    pub deaths: i32,
    pub tk: i32,
    pub spawn_frags: i32,
    pub kills: i32,
    pub suicides: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Weapons {
    pub sg: Weapon,
    pub ng: Weapon,
    pub ssg: Weapon,
    pub sng: Weapon,
    pub gl: Weapon,
    pub rl: Weapon,
    pub lg: Weapon,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Weapon {
    pub acc: WeaponAccuracy,
    pub kills: WeaponKills,
    pub deaths: i32,
    pub pickups: WeaponPickups,
    pub damage: WeaponDamage,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponAccuracy {
    pub attacks: i32,
    pub hits: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponDamage {
    pub enemy: i32,
    pub team: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponKills {
    pub total: i32,
    pub team: i32,
    pub enemy: i32,
    #[serde(rename = "self")]
    pub kills_self: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WeaponPickups {
    pub dropped: i32,
    pub taken: i32,
    pub total_taken: i32,
    pub spawn_taken: i32,
    pub spawn_total_taken: i32,
}
