
/// Configuration for everything you can to simplify initialization
pub struct Config {
    heroConfig: HeroConfig,
    gunConfig: GunConfig,
    gameConfig: GameConfig,
    mapConfig: MapConfig,
}

struct HeroConfig {
    health_max_value: u16,
}

struct GunConfig {

}

struct GameConfig {

}

struct MapConfig {

}

trait Configurated {
    type T
    fn from_config() -> T;
}