use super::hero::{Hero};
use super::geometry::{Direction};
use super::uniq::{ID};

/// Глобальное состояние игрока
#[derive(Debug)]
pub struct Player {
    pub id: PlayerID,
    pub hero: Hero,
}

/// Команды которые игроки могут отдавать своим героям
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum PlayerCommand {
    Move(Direction),
    Attack(Direction),
    Reload,
    Nothing,
}

pub type PlayerID = ID;