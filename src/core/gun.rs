
#[derive(Debug)]
/// Пример оружия дальнего боя
pub struct Bow{
    pub damage: u16,
}
/// Пример оружия ближнего боя
#[derive(Debug)]
pub struct Sword{
    pub damage: u16,
}
/// Пример оружия средней дистации
#[derive(Debug)]
pub struct Spear{
    pub damage: u16,
} 


/// Перечисление обобщающее все оружия
/// При создании нового оружия его необхожимо добавить сюда
#[derive(Debug)]
pub enum Gun {
    Bow(Bow),
    Sword(Sword),
    Srear(Spear),
}