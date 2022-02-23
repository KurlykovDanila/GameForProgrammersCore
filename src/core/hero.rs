use super::config::*;
use super::gun::*;
use super::uniq::*;
use std::cmp::min;

/// Состояния характеристик героя в игре
#[derive(Debug, Clone)]
pub struct Hero {
    id: PlayerId,
    health: Health,
    gun: Gun,
}

impl Uniq for Hero {
    fn id(&self) -> ID {
        return self.id;
    }
}

impl Armed for Gun {
    fn get_damage_value(&self) -> u16 {
        match self {
            Gun::Bow(bow) => {
                return bow.damage;
            }
            Gun::Srear(spear) => {
                return spear.damage;
            }
            Gun::Sword(sword) => {
                return sword.damage;
            }
        }
    }

    fn can_attack(&self) -> bool {
        match self {
            Gun::Bow(bow) => {
                return bow.reload.can_attack();
            }
            Gun::Srear(spear) => {
                return spear.reload.can_attack();
            }
            Gun::Sword(sword) => {
                return sword.reload.can_attack();
            }
        }
    }
    fn attack(&mut self) {
        match self {
            Gun::Bow(bow) => {
                bow.reload.attack();
            }
            Gun::Srear(spear) => {
                spear.reload.attack();
            }
            Gun::Sword(sword) => {
                sword.reload.attack();
            }
        }
    }
}

pub trait Armed {
    fn get_damage_value(&self) -> u16;
    fn can_attack(&self) -> bool;
    fn attack(&mut self);
}

impl Attack for Hero {
    fn attack(&mut self, target: &mut impl Mortal) {
        if self.gun.can_attack() {
            log::info!("Attack hero id: ({})", self.id());
            self.gun.attack();
            target.get_damage(self.gun.get_damage_value());
        }
        log::info!("Can not attack hero id: ({}), because reloading ({:?})", self.id(), self.gun);
    }
}

impl Mortal for Hero {
    fn get_damage(&mut self, value: u16) {
        log::info!("Get gamage hero id: ({})", self.id());
        self.health.reduce_current_value(value);
    }
}

pub trait Mortal {
    fn get_damage(&mut self, value: u16) -> ();
}

pub trait Attack {
    fn attack(&mut self, target: &mut impl Mortal) -> ();
}

impl Hero {
    pub fn new(health: u16, damage: u16, id: PlayerId) -> Hero {
        log::info!("Create hero id: ({})", id);
        let h = Health::new(health);
        return Hero {
            health: h,
            id: id,
            gun: Gun::Sword(Sword {
                damage: damage,
                range: 1,
                reload: Reloader::default(),
            }),
        };
    }

    pub fn from_config(config: &GameConfig, id: PlayerId) -> Hero {
        Hero {
            health: Health::from_config(config),
            id: id,
            gun: Gun::from_config(config),
        }
    }
}

/// Отвечает за состояние здоровья героя]
#[derive(Clone, Debug)]
struct Health {
    max_value: u16,
    current_value: u16,
}

impl Health {
    pub fn new(value: u16) -> Health {
        return Health {
            current_value: value,
            max_value: value,
        };
    }

    fn increase_current_value(&mut self, value: u16) {
        if u16::MAX - value <= self.current_value {
            self.current_value = self.max_value;
        } else {
            self.current_value = min(value + self.current_value, self.max_value);
        }
    }

    fn reduce_current_value(&mut self, value: u16) {
        if self.current_value < value {
            self.current_value = 0;
        } else {
            self.current_value -= value;
        }
    }

    fn increase_max_value(&mut self, value: u16) {
        if u16::MAX - value < self.max_value {
            self.max_value = u16::MAX;
        } else {
            self.max_value += value;
        }
    }

    fn reduce_max_value(&mut self, value: u16) {
        if self.max_value <= value {
            self.max_value = 1
        } else {
            self.max_value -= value;
        }
        self.normalize_current_value();
    }

    fn normalize_current_value(&mut self) {
        if self.max_value >= self.current_value {
            return;
        } else {
            self.current_value = self.max_value;
        }
    }
}

impl Configurated for Health {
    fn from_config(config: &GameConfig) -> Health {
        return Health::new(config.hero_max_health());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_test() {
        let mut health = Health::new(100);
        health.increase_current_value(100);
        assert_eq!(health.current_value, health.max_value);

        health.reduce_current_value(10);
        assert_eq!(health.current_value, 90);

        health.reduce_current_value(100);
        assert_eq!(health.current_value, 0);

        health.increase_max_value(u16::MAX);
        assert_eq!(health.max_value, u16::MAX);

        let mut health = Health::new(100);
        health.reduce_max_value(u16::MAX);
        assert_eq!(health.max_value, 1);
        assert_eq!(health.current_value, health.max_value);

        let mut health = Health::new(100);
        health.increase_max_value(u16::MAX);
        health.increase_current_value(u16::MAX);
        assert_eq!(health.current_value, health.max_value);
    }
}
