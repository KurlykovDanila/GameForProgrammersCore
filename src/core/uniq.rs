
pub mod uniq {
    /// УНИКАЛЬНЫЙ идентфикатор 
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct ID(pub u16);

}

pub mod player {
    use crate::uniq::uniq::{ID};
    use crate::hero::hero::{Hero};
    use crate::geometry::geometry::{Direction};

    /// Глобальное состояние игрока
    #[derive(Debug)]
    pub struct Player<'a> {
        pub id: ID,
        hero: Hero<'a>,
    }

    /// Команды которые игроки могут отдавать своим героям
    #[derive(Debug)]
    pub enum PlayerCommand {
        Move(Direction),
        Attack(Direction),
        Reload,
    }
}