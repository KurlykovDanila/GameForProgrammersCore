use std::cmp::Eq;
use std::hash::Hash;


/// УНИКАЛЬНЫЙ идентфикатор 
#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Hash)]
pub struct ID(pub u16);

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ID {}

struct GeneratorUnicID(Vec<ID>);

impl GeneratorUnicID {
    pub fn new_id(&mut self) -> ID {
        let id = ID(self.generate_uniq_id());
        self.0.push(id);
        return id;
    }

    fn generate_uniq_id(&self) -> u16 {
        // TODO;
        return 1;
    }
}
