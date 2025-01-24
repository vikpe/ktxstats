use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct KtxstatsV3 {
    pub version: u32,
    pub date: DateTime<Utc>,
    pub map: String,
    pub hostname: String,
    pub ip: String,
    pub port: u32,
    pub matchtag: String,
    pub mode: String,
    pub tl: u32,
    pub dm: u32,
    pub tp: u32,
    pub duration: u32,
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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
    pub ctf: Option<CtfStats>,
    pub bot: Option<BotConfig>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerSpeed {
    pub avg: f64,
    pub max: f64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Items {
    pub health_15: Health,
    pub health_25: Health,
    pub health_100: Health,
    pub ga: Armor,
    pub ya: Armor,
    pub ra: Armor,
    pub q: Powerup,
    pub r: Powerup,
    pub p: Powerup,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Armor {
    pub took: i32,
    pub time: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Powerup {
    pub took: i32,
    pub time: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BotConfig {
    pub skill: i32,
    pub customised: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CtfStats {
    pub points: i32,
    pub caps: i32,
    pub defends: i32,
    pub carrier_defends: i32,
    pub carrier_frags: i32,
    pub pickups: i32,
    pub returns: i32,
    pub runes: [i32; 4],
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Health {
    pub took: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FragSpree {
    pub max: i32,
    pub quad: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PlayerStats {
    pub frags: i32,
    pub deaths: i32,
    pub tk: i32,
    pub spawn_frags: i32,
    pub kills: i32,
    pub suicides: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Weapons {
    pub axe: Weapon,
    pub sg: Weapon,
    pub ng: Weapon,
    pub ssg: Weapon,
    pub sng: Weapon,
    pub gl: LauncherWeapon,
    pub rl: LauncherWeapon,
    pub lg: Weapon,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Weapon {
    pub acc: WeaponAccuracy,
    pub kills: WeaponKills,
    pub deaths: i32,
    pub pickups: WeaponPickups,
    pub damage: WeaponDamage,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherWeapon {
    pub acc: LauncherAccuracy,
    pub kills: WeaponKills,
    pub deaths: i32,
    pub pickups: WeaponPickups,
    pub damage: WeaponDamage,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponAccuracy {
    pub attacks: i32,
    pub hits: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherAccuracy {
    pub attacks: i32,
    pub hits: i32,
    pub real: i32,
    #[serde(rename = "virtual")]
    pub virt: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponDamage {
    pub enemy: i32,
    pub team: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponKills {
    pub total: i32,
    pub team: i32,
    pub enemy: i32,
    #[serde(rename = "self")]
    pub kills_self: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WeaponPickups {
    pub dropped: i32,
    pub taken: i32,
    pub total_taken: i32,
    pub spawn_taken: i32,
    pub spawn_total_taken: i32,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_try_from() {
        {
            // ctf
            let demo_content = fs::read_to_string(
                "tests/files/20240520-1925_ctf_blue_vs_red[ctf5].mvd.ktxstats.json",
            )
            .unwrap();
            let stats = KtxstatsV3::try_from(demo_content.as_str()).unwrap();

            assert_eq!(stats.version, 3);
            assert_eq!(
                stats.date.to_rfc3339(),
                "2024-05-20T19:35:42+00:00".to_string()
            );
            assert_eq!(stats.map, "ctf5".to_string());
            assert_eq!(stats.hostname, "qwctf.se:28501".to_string());
            assert_eq!(stats.ip, "127.0.1.1".to_string());
            assert_eq!(stats.port, 28501);
            assert_eq!(stats.matchtag, "".to_string());
            assert_eq!(stats.mode, "ctf".to_string());
            assert_eq!(stats.tl, 10);
            assert_eq!(stats.dm, 3);
            assert_eq!(stats.tp, 4);
            assert_eq!(stats.duration, 600);
            assert_eq!(
                stats.demo,
                "ctf_blue_vs_red[ctf5]20240520-1925.mvd".to_string()
            );

            assert_eq!(stats.teams, vec!["blue".to_string(), "red".to_string()]);
            let player = stats.players[0].clone();
            let ctfstats = player.ctf.unwrap_or_default();
            assert_eq!(ctfstats.points, 8);
            assert_eq!(ctfstats.caps, 0);
            assert_eq!(ctfstats.defends, 0);
            assert_eq!(ctfstats.carrier_defends, 4);
            assert_eq!(ctfstats.carrier_frags, 1);
            assert_eq!(ctfstats.pickups, 2);
            assert_eq!(ctfstats.returns, 0);
            assert_eq!(ctfstats.runes, [0, 110, 22, 192]);
        }
        {
            // 1on1
            let demo_content = fs::read_to_string(
                "tests/files/20250117-1613_1on1_p4to_vs_s1nistro[bravado].mvd.ktxstats.json",
            )
            .unwrap();
            let stats = KtxstatsV3::try_from(demo_content.as_str()).unwrap();

            assert_eq!(stats.version, 3);
            assert_eq!(
                stats.date.to_rfc3339(),
                "2025-01-17T16:23:40+00:00".to_string()
            );
            assert_eq!(stats.map, "bravado".to_string());
            assert_eq!(stats.hostname, "! ilha quad #0".to_string());
            assert_eq!(stats.ip, "127.0.0.1".to_string());
            assert_eq!(stats.port, 27500);
            assert_eq!(stats.matchtag, "".to_string());
            assert_eq!(stats.mode, "duel".to_string());
            assert_eq!(stats.tl, 10);
            assert_eq!(stats.dm, 3);
            assert_eq!(stats.tp, 0);
            assert_eq!(stats.duration, 600);
            assert_eq!(
                stats.demo,
                "duel_p4to_vs_s1nistro[bravado]20250117-1313.mvd".to_string()
            );
            assert_eq!(stats.teams.len(), 0);
            assert_eq!(stats.players.len(), 2);

            let player = stats.players[0].clone();
            assert_eq!(player.top_color, 4);
            assert_eq!(player.bottom_color, 4);
            assert_eq!(player.ping, 34);
            assert_eq!(player.login, "viper".to_string());
            assert_eq!(player.name, "p\u{0016}to".to_string());
            assert_eq!(player.team, "red".to_string());
            assert_eq!(
                player.stats,
                PlayerStats {
                    frags: 34,
                    deaths: 13,
                    tk: 0,
                    spawn_frags: 4,
                    kills: 36,
                    suicides: 2
                }
            );
            assert_eq!(
                player.dmg,
                PlayerDamage {
                    taken: 5560,
                    given: 7549,
                    team: 0,
                    dmg_self: 725,
                    team_weapons: 0,
                    enemy_weapons: 6548,
                    taken_to_die: 427
                }
            );
            assert_eq!(player.xfer_rl, 0);
            assert_eq!(player.xfer_lg, 0);
            assert_eq!(player.spree, FragSpree { max: 13, quad: 0 });
            assert_eq!(player.control, 313.233551);
            assert_eq!(
                player.speed,
                PlayerSpeed {
                    avg: 339.284119,
                    max: 1034.684204
                }
            );
            assert_eq!(
                player.weapons,
                Weapons {
                    axe: Weapon::default(),
                    sg: Weapon {
                        acc: WeaponAccuracy {
                            attacks: 126,
                            hits: 25
                        },
                        kills: WeaponKills::default(),
                        deaths: 0,
                        pickups: WeaponPickups {
                            dropped: 2,
                            taken: 0,
                            total_taken: 2,
                            spawn_taken: 0,
                            spawn_total_taken: 0
                        },
                        damage: WeaponDamage {
                            enemy: 100,
                            team: 0
                        },
                    },
                    ssg: Weapon {
                        acc: WeaponAccuracy {
                            attacks: 56,
                            hits: 29
                        },
                        kills: WeaponKills {
                            total: 1,
                            team: 0,
                            enemy: 10,
                            kills_self: 0
                        },
                        deaths: 0,
                        pickups: WeaponPickups {
                            dropped: 0,
                            taken: 5,
                            total_taken: 6,
                            spawn_taken: 5,
                            spawn_total_taken: 5
                        },
                        damage: WeaponDamage { enemy: 97, team: 0 },
                    },
                    ng: Weapon {
                        acc: WeaponAccuracy::default(),
                        kills: WeaponKills {
                            total: 0,
                            team: 0,
                            enemy: 5,
                            kills_self: 0
                        },
                        deaths: 0,
                        pickups: WeaponPickups {
                            dropped: 0,
                            taken: 1,
                            total_taken: 1,
                            spawn_taken: 1,
                            spawn_total_taken: 1
                        },
                        damage: WeaponDamage::default(),
                    },
                    sng: Weapon {
                        acc: WeaponAccuracy {
                            attacks: 9,
                            hits: 2
                        },
                        kills: WeaponKills {
                            total: 1,
                            team: 0,
                            enemy: 10,
                            kills_self: 0
                        },
                        deaths: 0,
                        pickups: WeaponPickups {
                            dropped: 0,
                            taken: 5,
                            total_taken: 6,
                            spawn_taken: 5,
                            spawn_total_taken: 5
                        },
                        damage: WeaponDamage { enemy: 25, team: 0 },
                    },
                    gl: LauncherWeapon {
                        acc: LauncherAccuracy {
                            attacks: 7,
                            hits: 1,
                            real: 1,
                            virt: 1,
                        },
                        kills: WeaponKills {
                            total: 0,
                            team: 0,
                            enemy: 12,
                            kills_self: 0
                        },
                        deaths: 1,
                        pickups: WeaponPickups {
                            dropped: 0,
                            taken: 7,
                            total_taken: 7,
                            spawn_taken: 7,
                            spawn_total_taken: 7
                        },
                        damage: WeaponDamage {
                            enemy: 105,
                            team: 0
                        },
                    },
                    rl: LauncherWeapon {
                        acc: LauncherAccuracy {
                            attacks: 210,
                            hits: 23,
                            real: 71,
                            virt: 71,
                        },
                        kills: WeaponKills {
                            total: 21,
                            team: 0,
                            enemy: 25,
                            kills_self: 2
                        },
                        deaths: 10,
                        pickups: WeaponPickups {
                            dropped: 9,
                            taken: 13,
                            total_taken: 27,
                            spawn_taken: 12,
                            spawn_total_taken: 12
                        },
                        damage: WeaponDamage {
                            enemy: 5186,
                            team: 0
                        },
                    },
                    lg: Weapon {
                        acc: WeaponAccuracy {
                            attacks: 263,
                            hits: 74
                        },
                        deaths: 0,
                        pickups: WeaponPickups {
                            dropped: 2,
                            taken: 8,
                            total_taken: 12,
                            spawn_taken: 6,
                            spawn_total_taken: 6
                        },
                        damage: WeaponDamage {
                            enemy: 2036,
                            team: 0
                        },
                        kills: WeaponKills {
                            total: 13,
                            team: 0,
                            enemy: 16,
                            kills_self: 0
                        }
                    }
                }
            );
            assert_eq!(
                player.items,
                Items {
                    health_15: Health { took: 2 },
                    health_25: Health { took: 32 },
                    health_100: Health { took: 7 },
                    ga: Armor { took: 2, time: 21 },
                    ya: Armor {
                        took: 12,
                        time: 174
                    },
                    ra: Armor {
                        took: 16,
                        time: 309
                    },
                    q: Powerup::default(),
                    p: Powerup::default(),
                    r: Powerup::default(),
                }
            )
        }
        {
            // 4on4
            let demo_content = fs::read_to_string(
                "tests/files/20250124-0556_4on4_blue_vs_red[dm3].mvd.ktxstats.json",
            )
            .unwrap();
            let stats = KtxstatsV3::try_from(demo_content.as_str()).unwrap();

            assert_eq!(stats.version, 3);
            assert_eq!(
                stats.date.to_rfc3339(),
                "2025-01-24T06:16:38+00:00".to_string()
            );
            assert_eq!(stats.map, "dm3".to_string());
            assert_eq!(stats.hostname, "la.quake.world:28501 NAQW".to_string());
            assert_eq!(stats.ip, "127.0.1.1".to_string());
            assert_eq!(stats.port, 28501);
            assert_eq!(stats.matchtag, "".to_string());
            assert_eq!(stats.mode, "team".to_string());
            assert_eq!(stats.tl, 20);
            assert_eq!(stats.dm, 1);
            assert_eq!(stats.tp, 2);
            assert_eq!(stats.duration, 1200);
            assert_eq!(
                stats.demo,
                "4on4_red_vs_blue[dm3]20250124-0556.mvd".to_string()
            );
            assert_eq!(stats.teams, vec!["red".to_string(), "blue".to_string()]);
            assert_eq!(stats.players.len(), 8);

            let player1 = stats.players[0].clone();
            assert_eq!(player1.items.q, Powerup { took: 3, time: 64 });
            assert_eq!(player1.items.r, Powerup { took: 2, time: 14 });
            assert_eq!(player1.items.p, Powerup { took: 1, time: 30 });
        }
    }
}
