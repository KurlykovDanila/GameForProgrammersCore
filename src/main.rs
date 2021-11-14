mod geometry;
mod map;
mod game;
mod uniq;
mod hero;

fn main() {
    let v = geometry::geometry::Vector2{x: 1, y: 1};
    println!("{:?}", v);
}