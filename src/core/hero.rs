use super::uniq::{ID};
use super::gun::{Gun};


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


