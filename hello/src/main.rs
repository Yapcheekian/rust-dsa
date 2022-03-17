extern crate rand;

use rand::Rng;
use std::ops::Add;

pub struct Person {
    name: String,
    age: i32,
    children: i32,
}

impl Person {
    pub fn print(&self) -> String {
        format!("name = {}, age = {}, children = {}", self.name, self.age, self.children)
    }
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Point {
    fn random() -> Self {
        let mut tr = rand::thread_rng();
        Point{
            x: tr.gen(),
            y: tr.gen(),
        }
    }
}

fn main() {
    let x = Person{
        name: "yap".to_string(),
        age: 18,
        children: 1,
    };

    println!("{}", x.print());

    let a = Point{x: 5, y: 6};
    let b = Point{x: 50, y: 60};
    let c = a + b;
    println!("c = {:?}", c);

    let d = Point::random();
    println!("d = {:?}", d);
}
