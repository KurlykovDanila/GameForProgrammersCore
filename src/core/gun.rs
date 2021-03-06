use crate::core::config::*;
use serde::{Deserialize, Serialize};

/// Пример оружия дальнего боя
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Bow {
    pub damage: u16,
    pub range: u8,
    pub reload: Reloader,
}
/// Пример оружия ближнего боя
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sword {
    pub damage: u16,
    pub range: u8,
    pub reload: Reloader,
}
/// Пример оружия средней дистации
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Spear {
    pub damage: u16,
    pub range: u8,
    pub reload: Reloader,
}

/// Перечисление обобщающее все оружия
/// При создании нового оружия его необхожимо добавить сюда
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Gun {
    Bow(Bow),
    Sword(Sword),
    Srear(Spear),
}

impl Configurated for Gun {
    fn from_config(config: &GameConfig) -> Self {
        return config.gun_type();
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Reloader {
    #[serde(rename = "max_time")]
    reload_time_max: u8,
    #[serde(rename = "current_time")]
    reload_time_now: u8,
    #[serde(rename = "max_amount_attacks")]
    size_max: u8,
    #[serde(rename = "current_amount_attacks")]
    size_current: u8,
}

impl Reloader {
    pub fn new(reload_time: u8, size: u8) -> Reloader {
        Reloader {
            reload_time_max: reload_time,
            reload_time_now: 0,
            size_max: size,
            size_current: size,
        }
    }

    pub fn can_attack(&self) -> bool {
        if self.reload_time_now == 0 && self.size_current > 0 {
            return true;
        }
        return false;
    }

    pub fn attack(&mut self) {
        if self.can_attack() {
            self.reload_time_now = self.reload_time_max;
            self.size_current -= 1;
            if !self.can_attack() {
                self.reload_time_now = self.reload_time_max;
                self.size_current = self.size_max;
            }
        }
        self.reload();
    }

    fn reload(&mut self) {
        self.reload_time_now -= 1;
    }
}

impl Default for Reloader {
    fn default() -> Reloader {
        Reloader::new(0, 1)
    }
}
