use super::uniq::{ID};
use super::player::{PlayerCommand, Player, PlayerID};
use super::map::{Map};
use std::cmp::{min};
use std::collections::HashMap;
use super::geometry::{Direction, Vector2};
use super::hero::Hero;

/// Связывает игроков, их героев и выбранную карту
/// Отвечает за выполнение ходов игроками
#[derive(Debug)]
pub struct Game<'a> {
    game_state: GameState,
    map: &'a mut Map,
    players: Vec<&'a mut Player>,
    shift: Shift
}

#[derive(Debug)]
enum GameState {
    NotStarted,
    Continue,
    Finished,
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
    pub fn new(players: Vec<&'a mut Player>, map: &'a mut Map) -> Game<'a>{
        let shift = Shift{current_value: 0, max_value: players.len() as u8};
        Game {
            game_state: GameState::NotStarted,
            map: map,
            players: players,
            shift: shift,
        }
    }
    /// Все приготовления перед игрой
    /// 
    /// Необходимо запустить перед `do_step`
    pub fn start(&mut self) {
        self.game_state = GameState::Continue;
        self.map.print();
    }
    

    /// Происходит один ход каждого игрока
    pub fn do_step(&mut self, commands: HashMap<PlayerID, Vec<PlayerCommand>>) {
        match self.game_state {
            // Нужно переработать, сделано не правильно
            GameState::Continue => {
                let max_len_commands = commands.values().max_by_key(|val| val.len()).unwrap().len();
                let players_vec: Vec<PlayerID> = commands.keys().cloned().collect::<Vec<PlayerID>>();
                for command_index in 0..(max_len_commands) {
                    for player_index in 0..players_vec.len() {
                        let player_id = players_vec[(player_index + self.shift.current_value as usize) % self.shift.max_value as usize];
                        let hero_id: ID = self.player_by_player_id(player_id).unwrap().hero.id;
                        if commands[&player_id].len() > command_index {
                            self.execute_player_command(hero_id, commands[&player_id][command_index])
                        }
                    }
                }
                self.shift.next();
            },
            _ => {
                return;
            }
        }
    }

    fn player_by_player_id(&self, player_id: PlayerID) -> Option<&Player> {
        Option::Some(*self.players.iter().find(|player| (***player).id == player_id).unwrap())
    }

    /// Завршение игры по причине победы игрока или принудительно из-за каких либо проблем
    /// 
    /// (Корректное завершение игры)
    pub fn end(&mut self) {
        self.game_state = GameState::Finished;
    }

    /// Парсинг команды игрока и её применение к герою со всеми проверками 
    
    fn execute_player_command(&mut self, hero_id: ID, player_command: PlayerCommand) {
        match player_command {
            PlayerCommand::Attack(direction) => {},
            PlayerCommand::Move(direction) => {
                self.map.move_hero(hero_id, &direction);
            },
            PlayerCommand::Reload => {},
            PlayerCommand::Nothing => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn game_do_step_test() {
        let commands = HashMap::from([
            (
                ID(1), 
                vec![
                    PlayerCommand::Move(Direction::Top), 
                    PlayerCommand::Move(Direction::Left),
                    PlayerCommand::Move(Direction::Left),
                ]
            ),
            (
                ID(0),
                vec![
                    PlayerCommand::Move(Direction::Bottom),
                    PlayerCommand::Move(Direction::Top),
                ]
            )
        ]);
        let mut map = Map::new(5);
        let mut hero1 = Hero::new(100, 10, ID(0));
        let mut player1 = Player{id: ID(0), hero: hero1};
        let mut hero2 = Hero::new(100, 10, ID(1));
        let mut player2 = Player{id: ID(1), hero: hero2};
        map.add_hero(hero1, &Vector2{x: 2, y: 2});
        map.add_hero(hero2, &Vector2{x: 1, y: 1});
        let mut game = Game::new(vec![&mut player1, &mut player2], &mut map);
        game.start();
        let commands2 = commands.clone();
        game.do_step(commands);
    }
}