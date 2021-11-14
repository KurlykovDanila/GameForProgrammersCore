/// УНИКАЛЬНЫЙ идентфикатор 
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ID(pub u16);

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
