#[allow(dead_code)]
mod core;
use crate::core::api::*;
use crate::core::game::*;
use crate::core::geometry::*;
use crate::core::hero::*;
use crate::core::map::*;
use crate::core::player::*;
use crate::core::uniq::*;
use envconfig::Envconfig;

fn main() {
    simple_logger::init_with_level(log::Level::Trace).unwrap();
    let mut generator_id = GeneratorUnicID::new();
    let player1 = Player::new_by_id(generator_id.new_id());
    let player2 = Player::new_by_id(generator_id.new_id());
    let mut commands1 = PlayerCommands::new(3);
    commands1.add(PlayerCommand::Move(Direction::Top));
    commands1.add(PlayerCommand::Move(Direction::Left));
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
