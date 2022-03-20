use super::geometry::Direction;
use super::hero::Hero;
use super::statistic::Statistics;
use super::uniq::*;
use std::fmt;

/// Глобальное состояние игрока
#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    pub hero: Hero,
    pub statistic: Statistics,
}

/// Команды которые игроки могут отдавать своим героям
#[derive(Copy, Clone, Debug)]
pub enum PlayerCommand {
    Move(Direction),
    Attack(Direction),
    Reload,
    Nothing,
}

pub struct PlayerCommands {
    index: usize,
    commands: Vec<PlayerCommand>,
}

impl PlayerCommands {
    pub fn new(len: usize) -> PlayerCommands {
        PlayerCommands {
            index: 0,
            commands: vec![PlayerCommand::Nothing; len],
        }
    }

    pub fn add(&mut self, command: PlayerCommand) {
        if self.index < self.commands.len() {
            self.commands[self.index] = command;
            self.index += 1;
        }
    }
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn get_command(&self, index: usize) -> PlayerCommand {
        if index < self.commands.len() {
            return self.commands[index];
        }
        return PlayerCommand::Nothing;
    }
}

impl fmt::Display for PlayerCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.commands)
    }
}

impl Uniq for Player {
    fn id(&self) -> PlayerId {
        return self.id;
    }
}

impl Player {
    fn new(hero: Hero, id: PlayerId) -> Player {
        let player = Player {
            hero: hero,
            id: id,
            statistic: Statistics::default(),
        };
        log::info!("Create new player: {:?}", player);
        return player;
    }

    pub fn new_by_id(id: PlayerId) -> Player {
        let player = Player {
            hero: Hero::new(100, 10, id),
            id: id,
            statistic: Statistics::default(),
        };
        log::info!("Create new player: {:?}", player);
        return player;
    }
}
