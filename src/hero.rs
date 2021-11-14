
pub mod gun {
    
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
}

pub mod hero {
    use crate::uniq::uniq::{ID};
    use crate::hero::gun::{Gun};

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

}

