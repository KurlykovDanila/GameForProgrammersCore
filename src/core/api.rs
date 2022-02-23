use crate::core::config::*;
use crate::core::game::*;
use crate::core::geometry::*;
use crate::core::hero::*;
use crate::core::map::*;
use crate::core::player::*;
use crate::core::uniq::*;
use envconfig::Envconfig;

pub struct API {
    player_id_generator: GeneratorUnicID,
    game_id_generator: GeneratorUnicID,
    games: Vec<Game>,
    players: Vec<Player>,
    config: GameConfig,
}

impl API {
    pub fn new() -> API {
        API {
            player_id_generator: GeneratorUnicID::new(),
            game_id_generator: GeneratorUnicID::new(),
            games: Vec::new(),
            players: Vec::new(),
            config: GameConfig::init_from_env().unwrap(),
        }
    }
}
