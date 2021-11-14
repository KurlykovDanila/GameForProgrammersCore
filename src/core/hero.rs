use super::uniq::{ID};
use super::gun::{Gun};
use std::cmp::{min};


/// Состояния характеристик героя в игре
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Hero<'a>{
    pub id: ID,
    health: &'a Health,
    gun: &'a Gun,

}
/// Отвечает за состояние здоровья героя
#[derive(Copy, Clone)]
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


