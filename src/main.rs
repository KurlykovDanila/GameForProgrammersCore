#[allow(dead_code)]
mod core;
mod server;
use crate::core::api::*;
use crate::core::game::*;
use crate::core::geometry::*;
use crate::core::hero::*;
use crate::core::map::*;
use crate::core::player::*;
use crate::core::uniq::*;
use envconfig::Envconfig;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use crate::server::server::*;

fn init_logger() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d}][{l}][{t}] - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();
}



fn main() {
    init_logger(); 
    let sign = Sign {
        typ: SignType::SignIn,
        username: String::from("Danila"),
        password: String::from("asdasd123"),
    };
    println!("{}", serde_json::to_string_pretty(&sign).unwrap_or(String::new()));

    let s: Sign = serde_json::from_str(r#"{
        "type": "SignIn",
        "username": "Danila",
        "password": "asdasd123"
      }"#).unwrap();
    
    println!("{}", s.username);

    let mut server = Server{};
    server.run();

    // let command1 =  PlayerCommand::Attack(Direction::Left);

    // let command2 =  PlayerCommand::Reload;

    // let mut players_commands = PlayerCommands::new(3);
    // players_commands.add(command1);
    // players_commands.add(command2);

    // let hero1 = Hero::new(1000, 70, ID(171));
    // let hero2 = Hero::new(1000, 70, ID(3831));
    // let mut map = Map::new(4);
    // map.add_hero(hero1, Vector2::new(0, 0));
    // map.add_hero(hero2, Vector2::new(3, 3));
    // println!("{}", serde_json::to_string_pretty(&map).unwrap_or(String::new()));



    // let mut server = Server{};
    // server.run();

    // let mut generator_id = GeneratorUnicID::new();
    // let player1 = Player::new_by_id(generator_id.new_id());
    // let player2 = Player::new_by_id(generator_id.new_id());
    // let mut commands1 = PlayerCommands::new(3);
    // commands1.add(PlayerCommand::Move(Direction::Top));
    // commands1.add(PlayerCommand::Move(Direction::Left));
    // let mut commands2 = PlayerCommands::new(3);
    // commands2.add(PlayerCommand::Move(Direction::Bottom));
    // commands2.add(PlayerCommand::Move(Direction::Top));
    // commands2.add(PlayerCommand::Attack(Direction::Top));
    // let players_commands = vec![(player1.id(), commands1), (player2.id(), commands2)];
    // let mut map = Map::new(5);
    // map.add_hero(player1.hero.clone(), Vector2 { x: 0, y: 0 });
    // map.add_hero(player2.hero.clone(), Vector2 { x: 4, y: 4 });
    // let mut game = Game::new(vec![player1.hero.clone(), player2.hero.clone()], map, ID(0));
    // game.start();
    // game.do_step(players_commands);
}
