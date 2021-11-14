use super::hero::{Hero};
use super::geometry::{Direction};
use super::uniq::{ID};

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