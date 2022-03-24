use std::cmp::Eq;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

/// УНИКАЛЬНЫЙ идентфикатор
#[derive(Copy, Clone, Debug, Hash)]

#[derive(Serialize, Deserialize)]
pub struct ID(pub u16);
pub type PlayerId = ID;
pub type GameId = ID;
pub type MapId = ID;

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ID {}

pub struct GeneratorUnicID {
    free_values: Vec<bool>,
}

impl GeneratorUnicID {
    pub fn new() -> GeneratorUnicID {
        GeneratorUnicID {
            free_values: vec![true; std::u16::MAX.into()],
        }
    }

    pub fn new_id(&mut self) -> ID  {
        let id_value = self.free_values.iter().position(|x| *x == true).unwrap();
        self.free_values[id_value] = false;
        return ID(id_value as u16);
    }

    pub fn free_id(&mut self, id: ID) {
        self.free_values[id.0 as usize] = true;
    }
}

pub trait Uniq {
    fn id(&self) -> ID;
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
