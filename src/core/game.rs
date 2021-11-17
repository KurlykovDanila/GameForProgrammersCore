use super::uniq::{ID};
use super::player::{PlayerCommand, Player};
use super::map::{Map};
use std::cmp::{min};
use std::collections::HashMap;

/// Связывает игроков, их героев и выбранную карту
/// Отвечает за выполнение ходов игроками
#[derive(Debug)]
pub struct Game<'a> {
    map: &'a mut Map<'a>,
    players: Vec<&'a mut Player>,
    shift: Shift
}

    
/// Игра каждый ход определяет игрока который начинает ход простым сдвигом
/// # Пример 
/// ```
/// players = [p1, p2, p3]
/// ```
/// В первый ход shift = 0 и ход начинается с первого игрока
/// 
/// Во второй ход shift = 1 и ход начинается со второго
/// 
/// Аналлгично с третьим ходом
/// 
///  На четвертый же ход shit обнуляется и всё начинается занаво
#[derive(Debug)]
struct Shift {
    current_value: u8,
    max_value: u8,
}

impl Shift {
    pub fn next(&mut self) {
        self.current_value = (self.current_value + 1) % self.max_value
    }

    pub fn increase_max_value(&mut self, value: u8) {
        self.max_value += value;
        self.normalize_current_value();
    }
    
    pub fn reduce_max_value(&mut self, value: u8) {
        self.max_value -= value;
        self.normalize_current_value();
    }

    fn normalize_current_value(&mut self) {
        self.current_value = min(self.current_value, self.max_value);
    }

}

impl<'a> Game<'a> {

    /// Создание новой игры
    pub fn new(players: Vec<&'a mut Player>, map: &'a mut Map<'a>) -> Game<'a>{
        let shift = Shift{current_value: 0, max_value: players.len() as u8};
        Game {
            map: map,
            players: players,
            shift: shift,
        }
    }
    /// Все приготовления перед игрой
    /// 
    /// Необходимо запустить перед `do_step`
    pub fn start(&mut self) {}

    /// Происходит один ход каждого игрока
    pub fn do_step(&mut self, commands: HashMap<ID, Vec<PlayerCommand>>) {
        for pl in 0..(self.players.len() - 1) {
            let id = self.players[(pl + self.shift.current_value as usize) % (self.shift.max_value as usize)].hero.id;
            let or: Vec<PlayerCommand> = Vec::new();
            let pl_commands = commands.get(&id).unwrap_or(&or);
            for command in pl_commands {
                self.execute_player_command(id, command.clone());
            }
        }
    }

    /// Завршение игры по причине победы игрока или принудительно из-за каких либо проблем
    /// 
    /// (Корректное завершение игры)
    pub fn end(&mut self) {}

    /// Парсинг команды игрока и её применение к герою со всеми проверками 
    
    fn execute_player_command(&mut self, hero_id: ID, player_command: PlayerCommand) {
        //TODO
        match player_command {
            PlayerCommand::Attack(direction) => {},
            PlayerCommand::Move(direction) => {
                self.map.move_hero(hero_id, &direction);
            },
            PlayerCommand::Reload => {},
        }
    }
}

