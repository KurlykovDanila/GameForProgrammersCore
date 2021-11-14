use super::uniq::{ID};
use super::gun::{Gun};
use std::cmp::{min};


/// Состояния характеристик героя в игре
#[derive(Debug)]
pub struct Hero<'a>{
    pub id: ID,
    health: &'a mut Health,
    gun: &'a Gun,

}

impl Armed for Gun {
   fn get_damage_value(&self) -> u16 {
       match self {
           Gun::Bow(bow) => {
               return bow.damage;
           },
           Gun::Srear(spear) => {
               return spear.damage;
           },
           Gun::Sword(sword) => {
               return sword.damage;
           }
       }
   } 
}

pub trait Armed {
    fn get_damage_value(&self) -> u16;
}

impl<'a> Attack for Hero<'a> {
    fn attack(&self, target: &mut impl Mortal) {
        target.get_damage(self.gun.get_damage_value());
    }
}

impl<'a> Mortal for Hero<'a> {
    fn get_damage(&mut self, value: u16) {
        self.health.reduce_current_value(value);
    }
} 

pub trait Mortal {
    fn get_damage(&mut self, value: u16) -> ();
}

pub trait Attack {

    fn attack(&self, target: &mut impl Mortal) -> ();
}

/// Отвечает за состояние здоровья героя]
#[derive(Debug)]
struct Health {
    max_value: u16,
    current_value: u16,
}

impl Health {
    pub fn new(value: u16) -> Health {
        return Health{current_value: value, max_value: value};
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
            self.current_value = 0
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
            return
        } else {
            self.current_value = self.max_value;
        }
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


