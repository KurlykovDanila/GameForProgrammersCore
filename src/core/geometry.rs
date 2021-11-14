

use std::marker::Copy;
use std::ops::Add;
/// Обычный вектор
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: i16,
    pub y: i16,
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{x: self.x + other.x, y: self.y + other.y}
    }
}

/// Просто направления 
#[derive(Debug)]
pub enum Direction {
    Top,
    Bottom,
    Right,
    Left
}

impl Direction {
    pub fn to_vector2(&self) -> Vector2 {
        match self {
            Top => Vector2{x: 0, y: 1},
            Bottom => Vector2{x: 0, y: -1},
            Right => Vector2{x: 1, y: 0},
            Left => Vector2{x: -1, y: 0},
        } 
    }
}
