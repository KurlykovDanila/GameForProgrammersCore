use super::geometry::{Direction, Vector2};
use super::hero::*;
use super::uniq::*;
use rand::prelude::*;
use std::fmt;
use serde::{Deserialize, Serialize};

/// Каждая клетка, которая может быть размещена на карте, должна быть внесена в данное перечисление
///
/// Через перечисление должна быть возможноть получить все необходимые данные об объекте в данной клетке, а лучше сам объект
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub enum CellType {
    Empty,
    Hero(Hero),
    Wall,
}


#[derive(Serialize, Deserialize)]
pub struct Map {
    size: u8,
    field: Vec<Vec<CellType>>,
    #[serde(skip)]
    heroes_coordinates: Vec<(ID, Vector2)>,
}

impl Map {
    /// Создание карты
    pub fn new(size: u8) -> Map {
        let mut rng = rand::thread_rng();
        let mut field: Vec<Vec<CellType>> =
            vec![vec![CellType::Empty; size as usize]; size as usize];
        for x in 0..field.len() {
            for y in 0..field.len() {
                if rng.gen::<f64>() < 0.3 {
                    field[x][y] = CellType::Wall;
                }
            }
        }
        field[0][0] = CellType::Empty;
        field[(size - 1) as usize][(size - 1) as usize] = CellType::Empty;
        log::trace!("Create map size: ({})", size);
        Map {
            size: size,
            field: field,
            heroes_coordinates: Vec::new(),
        }
    }

    pub fn attack(&mut self, hero_id: PlayerId, direction: Direction) {
        let hero_coordinate = self.get_hero_coord_by_id(hero_id);
        match hero_coordinate {
            Some(coordinate) => {
                match self.field[coordinate.x as usize][coordinate.y as usize].clone() {
                    CellType::Hero(mut attacker) => {
                        let attack_coordinate = coordinate + direction.to_vector2();
                        if !self.coordinate_on_map(attack_coordinate) {
                            log::trace!(
                                "Hero with id: ({}) attack out of map, attack coordinate: ({:?})",
                                attacker.id(),
                                attack_coordinate
                            );
                            return;
                        }
                        match self.field[attack_coordinate.x as usize][attack_coordinate.y as usize]
                            .clone()
                        {
                            CellType::Hero(mut enemy) => {
                                attacker.attack(&mut enemy);
                                log::info!(
                                    "Hero with id: ({}), attack hero with id: ({})",
                                    attacker.id(),
                                    enemy.id()
                                );
                                log::trace!("Attacker state: {:?}", attacker);
                                log::trace!("Enemy state: {:?}", enemy);
                            }
                            _ => {
                                log::info!("By coord: ({:?}) no enemy", attack_coordinate);
                            }
                        }
                    }
                    _ => {
                        log::warn!(
                            "By coord: ({:?}) not hero with id: ({})",
                            coordinate,
                            hero_id
                        );
                    }
                }
            }
            _ => {
                log::warn!("Hero not found with id: ({})", hero_id);
            }
        }
    }

    fn coordinate_on_map(&self, coordinate: Vector2) -> bool {
        if !(coordinate.x >= 0
            && (coordinate.x as usize) < self.size as usize
            && coordinate.y >= 0
            && (coordinate.y as usize) < self.size as usize)
        {
            return false;
        }
        return true;
    }

    /// Добавляет персонажа по внесённым координатам на карту если выбранная координата соответсвует `CellType::Empty`
    pub fn add_hero(&mut self, hero: Hero, coordinate: Vector2) -> bool {
        if !self.coordinate_on_map(coordinate) {
            log::warn!(
                "Try add hero with id: ({}) on coordinate: ({:?}) out of map",
                hero.id(),
                coordinate
            );
            return false;
        }
        match self.field[coordinate.x as usize][coordinate.y as usize].clone() {
            CellType::Empty => {
                log::info!(
                    "Add hero with id: ({}) on map with coord ({:?})",
                    hero.id(),
                    coordinate
                );
                self.heroes_coordinates.push((hero.id(), coordinate));
                self.change_cell_type(&coordinate, CellType::Hero(hero));
                return true;
            }
            cell => {
                log::warn!(
                    "Hero with id: ({}) not added on map, cell type was ({})",
                    hero.id(),
                    cell
                );
                return false;
            }
        }
    }

    fn change_cell_type(&mut self, coordinate: &Vector2, new_cell_type: CellType) {
        log::trace!(
            "Change cell type coord: ({:?}), from ({}) to ({})",
            *coordinate,
            self.field[coordinate.x as usize][coordinate.y as usize],
            new_cell_type
        );
        self.field[coordinate.x as usize][coordinate.y as usize] = new_cell_type;
    }

    fn get_hero_coord_by_id(&self, hero_id: PlayerId) -> Option<Vector2> {
        for i in self.heroes_coordinates.iter() {
            if i.0 == hero_id {
                return Some(i.1);
            }
        }
        return Option::None;
    }

    //pub fn get_hero_by_id(&self, id: HeroId) {} //-> &Hero

    /// Проверка на то что герой может походить в выбранном напрвлении
    fn hero_can_move(&self, heroes_coordinates: &Vector2, direction: &Direction) -> bool {
        let coord = *heroes_coordinates + direction.to_vector2();
        if !self.coordinate_on_map(coord) {
            return false;
        }
        match self.field[coord.x as usize][coord.y as usize] {
            CellType::Empty => {
                return true;
            }
            _ => false,
        }
    }

    pub fn move_hero(&mut self, hero_id: ID, direction: Direction) {
        let hero_coordinate = self.get_hero_coord_by_id(hero_id);
        match hero_coordinate {
            Some(coordinate) => {
                if self.hero_can_move(&coordinate, &direction) {
                    let new_coordinate = coordinate + direction.to_vector2();
                    let cell_type =
                        self.field[coordinate.x as usize][coordinate.y as usize].clone();
                    self.change_cell_type(&new_coordinate, cell_type);
                    self.change_cell_type(&coordinate, CellType::Empty);
                    self.change_hero_coordinate(hero_id, new_coordinate);
                    log::trace!(
                        "Move hero id: ({}), from ({:?}) to ({:?})",
                        hero_id,
                        coordinate,
                        new_coordinate
                    );
                } else {
                    log::trace!("Hero with id: ({}) can not move", hero_id);
                }
            }
            None => {
                log::warn!("Hero with id: ({}) not found", hero_id);
            }
        }
    }

    fn change_hero_coordinate(&mut self, id: ID, new_coordinate: Vector2) {
        for i in 0..self.heroes_coordinates.len() {
            if self.heroes_coordinates[i].0 .0 == id.0 {
                self.heroes_coordinates[i].1 = new_coordinate;
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                write!(f, "|{}", self.field[x as usize][y as usize]);
            }
            writeln!(f, "|");
        }
        Ok(())
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellType::Empty => "Empty",
                CellType::Hero(_) =>  "Hero ",
                CellType::Wall => "Wall ",
            }
        )
    }
}
