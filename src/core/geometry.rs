use serde::{Deserialize, Serialize};
use std::marker::Copy;
use std::ops::Add;

/// Обычный вектор
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: i16,
    pub y: i16,
}

impl Vector2 {
    pub fn new(x: i16, y: i16) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Просто направления
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "direction")]
pub enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}

impl Direction {
    pub fn to_vector2(&self) -> Vector2 {
        match self {
            Direction::Top => Vector2 { x: 0, y: 1 },
            Direction::Bottom => Vector2 { x: 0, y: -1 },
            Direction::Right => Vector2 { x: 1, y: 0 },
            Direction::Left => Vector2 { x: -1, y: 0 },
        }
    }
}
