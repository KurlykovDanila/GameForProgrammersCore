mod core;
use crate::core::hero::{Attack, Mortal};

fn main() {
    let v = core::geometry::Vector2{x: 1, y: 1};
    println!("{:?}", v);

    let mut map = core::map::Map::new(5);
    let hero = core::hero::Hero::new(100, 10, core::uniq::ID(1));
    let s_hero = core::hero::Hero::new(100, 10, core::uniq::ID(2));
    map.print();
    map.add_hero(hero, &core::geometry::Vector2{x: 2, y: 1});
    map.add_hero(s_hero, &core::geometry::Vector2{x: 3, y: 3});
    println!();
    map.print();
}
