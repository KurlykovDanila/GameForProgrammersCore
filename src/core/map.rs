use super::hero::{Hero};
use super::geometry::{Vector2, Direction};
use super::uniq::{ID};
use std::fmt;


/// Каждая клетка, которая может быть размещена на карте, должна быть внесена в данное перечисление
/// 
/// Через перечисление должна быть возможноть получить все необходимые данные об объекте в данной клетке, а лучше сам объект
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum CellType {
    Empty,
    Hero(Hero),
    Wall,
}

#[derive(Debug)]
pub struct Map {
    size: u8,
    field: Vec<Vec<CellType>>,
    heroes_coordinates: Vec<(ID, Vector2)>
}

impl Map {

    /// Создание карты
    pub fn new(size: u8) -> Map{
        let field: Vec<Vec<CellType>> = vec![vec![CellType::Empty; size as usize]; size as usize];
        Map {
            size: size,
            field: field,
            heroes_coordinates: Vec::new()
        }
    }

    /// Добавляет персонажа по внесённым координатам на карту если выбранная координата соответсвует `CellType::Empty`
    pub fn add_hero(&mut self, hero: Hero, coordinate: &Vector2) {
        match self.field[coordinate.x as usize][coordinate.y as usize] {
            CellType::Empty => {
                self.field[coordinate.x as usize][coordinate.y as usize] = CellType::Hero(hero);
                self.heroes_coordinates.push((hero.id, *coordinate));
            },
            _ => ()
        }
    }


    fn change_cell_type(&mut self, coordinate: &Vector2, new_cell_type: CellType) {
        self.field[coordinate.x as usize][coordinate.y as usize] = new_cell_type;
    }

    fn get_hero_coord_by_id(&self, hero_id: ID) -> Option<Vector2> {
        for i in self.heroes_coordinates.iter() {
            if i.0.0 == hero_id.0 {
                return Some(i.1);
            }
        }
        return Option::None;
    }

    pub fn get_hero_by_id(&self, id: ID) {}//-> &Hero


    /// Проверка на то что герой может  походить в выбранном напрвлении
    fn hero_can_move(&self, heroes_coordinates: &Vector2, direction: &Direction) -> bool {
        let coord = *heroes_coordinates + direction.to_vector2();
        if !(coord.x >= 0 && (coord.x as usize) < self.field.len() && coord.y >= 0 && (coord.y as usize) < self.field[coord.x as usize].len()) { 
            return false;
        }
        match self.field[coord.x as usize][coord.y as usize] {
            CellType::Empty => {
                return true;
            },
            _ => false,
        }
    }

    pub fn move_hero(&mut self, hero_id: ID, direction: &Direction) {
        let hero_coordinate = self.get_hero_coord_by_id(hero_id);
                match hero_coordinate {
                    Some(coordinate) => {
                        if self.hero_can_move(&coordinate, &direction) {
                            let new_coordinate = coordinate + direction.to_vector2();
                            let cell_type = self.field[coordinate.x as usize][coordinate.y as usize];
                            self.change_cell_type(&new_coordinate, cell_type);
                            self.change_cell_type(&coordinate, CellType::Empty);
                            self.change_hero_coordinate(hero_id, new_coordinate);
                        }
                    },
                    None => {},
                }
    }

    fn change_hero_coordinate(&mut self, id: ID, new_coordinate: Vector2) {
        for i in 0..self.heroes_coordinates.len() {
            if self.heroes_coordinates[i].0.0 == id.0 {
                self.heroes_coordinates[i].1 = new_coordinate;
            }
        }
    }

    pub fn print(&self) {
        println!();
        for i in 0..self.size {
            println!("{:?}", self.field[i as usize]);
        }
        println!();
    }

}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CellType::Empty => "Empty",
            CellType::Hero(_) => "Hero",
            CellType::Wall => "Wall"
        })
    }
}
