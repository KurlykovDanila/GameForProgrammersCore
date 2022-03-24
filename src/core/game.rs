use super::geometry::*;
use super::hero::*;
use super::map::*;
use super::player::*;
use super::uniq::*;
use log::*;
use std::cmp::min;

/// Связывает игроков, их героев и выбранную карту
/// Отвечает за выполнение ходов игроками
pub struct Game {
    id: GameId,
    game_state: GameState,
    map: Map,
    heroes: Vec<Hero>,
    shift: Shift,
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

impl Uniq for Game {
    fn id(&self) -> GameId {
        return self.id;
    }
}

impl Game {
    /// Создание новой игры
    pub fn new(heroes: Vec<Hero>, map: Map, id: GameId) -> Game {
        let shift = Shift {
            current_value: 0,
            max_value: heroes.len() as u8,
        };
        info!("Create new game with heroes: {:?}", heroes);
        Game {
            id: id,
            game_state: GameState::NotStarted,
            map: map,
            heroes: heroes,
            shift: shift,
        }
    }

    pub fn get_map(&self) -> &Map {
        return &self.map;
    }

    /// Все приготовления перед игрой
    ///
    /// Необходимо запустить перед `do_step`
    pub fn start(&mut self) {
        log::trace!("Game start");
        self.game_state = GameState::Continue;
    }

    /// Происходит один ход каждого игрока
    pub fn do_step(&mut self, commands: Vec<(PlayerId, PlayerCommands)>) {
        match self.game_state {
            // Нужно переработать, сделано не правильно
            GameState::Continue => {
                log::trace!("Game do step");
                let commands_len = commands[0].1.len();
                let players_vec: Vec<PlayerId> = commands.iter().map(|pair| pair.0).collect();
                for command_index in 0..(commands_len) {
                    for player_index in 0..players_vec.len() {
                        let player_id = players_vec[(player_index
                            + self.shift.current_value as usize)
                            % players_vec.len() as usize];
                        self.execute_player_command(
                            player_id,
                            commands[player_index].1.get_command(command_index),
                        );
                    }
                }
                self.shift.next();
            }
            _ => {
                return;
            }
        }
    }

    /// Завершение игры по причине победы игрока или принудительно из-за каких либо проблем
    ///
    /// (Корректное завершение игры)
    pub fn end(&mut self) {
        self.game_state = GameState::Finished;
    }

    /// Парсинг команды игрока и её применение к герою со всеми проверками
    fn execute_player_command(&mut self, hero_id: PlayerId, player_command: PlayerCommand) {
        match player_command {
            PlayerCommand::Attack(direction) => {
                log::trace!("Plyer attack execute for hero with id: ({})", hero_id);
                self.map.attack(hero_id, direction)
            }
            PlayerCommand::Move(direction) => {
                log::trace!("Plyer move execute for hero with id: ({})", hero_id);
                self.map.move_hero(hero_id, direction);
            }
            PlayerCommand::Reload => {}
            PlayerCommand::Nothing => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_do_step_test() {
        let mut generator_id = GeneratorUnicID::new();
        let player1 = Player::new_by_id(generator_id.new_id());
        let player2 = Player::new_by_id(generator_id.new_id());
        let mut commands1 = PlayerCommands::new(3);
        commands1.add(PlayerCommand::Move(Direction::Top));
        commands1.add(PlayerCommand::Move(Direction::Left));
        commands1.add(PlayerCommand::Move(Direction::Bottom));
        let mut commands2 = PlayerCommands::new(3);
        commands2.add(PlayerCommand::Move(Direction::Bottom));
        commands2.add(PlayerCommand::Move(Direction::Top));
        commands2.add(PlayerCommand::Attack(Direction::Top));
        let players_commands = vec![(player1.id(), commands1), (player2.id(), commands2)];
        let mut map = Map::new(5);

        map.add_hero(player1.hero.clone(), Vector2 { x: 0, y: 0 });
        map.add_hero(player2.hero.clone(), Vector2 { x: 4, y: 4 });
        let mut game = Game::new(vec![player1.hero.clone(), player2.hero.clone()], map, ID(0));
        game.start();
        game.do_step(players_commands);
        println!("{}", game.get_map());
    }
}
