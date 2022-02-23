use crate::core::gun::*;
use envconfig::Envconfig;

/// Configuration for everything you can to simplify initialization
#[derive(Envconfig)]
pub struct GameConfig {
    #[envconfig(from = "MAX_HEALTH", default = "100")]
    hero_health_max_value: u16,
    #[envconfig(from = "GUN_TYPE", default = "1")]
    gun_type: u8,
    #[envconfig(from = "GUN_DAMAGE", default = "15")]
    gun_damage: u16,
    #[envconfig(from = "MAP_SIZE", default = "5")]
    map_size: u8,
}

pub trait Configurated {
    fn from_config(config: &GameConfig) -> Self;
}

impl GameConfig {
    pub fn map_size(&self) -> u8 {
        self.map_size
    }

    pub fn hero_max_health(&self) -> u16 {
        self.hero_health_max_value
    }

    pub fn gun_type(&self) -> Gun {
        if self.gun_type == 0 {
            return Gun::Bow(Bow {
                damage: self.gun_damage,
                range: 3,
                reload: Reloader::new(3, 5),
            });
        }
        if self.gun_type == 1 {
            return Gun::Sword(Sword {
                damage: self.gun_damage,
                range: 1,
                reload: Reloader::default()
            });
        }
        return Gun::Srear(Spear {
            damage: self.gun_damage,
            range: 2,
            reload: Reloader::new(1, 1)
        });
    }
}
