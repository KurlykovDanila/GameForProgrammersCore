
#[derive(Copy, Clone)]
#[derive(Debug)]
/// Пример оружия дальнего боя
pub struct Bow{}
/// Пример оружия ближнего боя
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Sword{}
/// Пример оружия средней дистации
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Spear{} 


/// Перечисление обобщающее все оружия
/// При создании нового оружия его необхожимо добавить сюда
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Gun {
    Bow(Bow),
    Sword(Sword),
    Srear(Spear),
}