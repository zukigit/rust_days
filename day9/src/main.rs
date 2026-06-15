use std::{fmt, ops::Add};

trait Animal {
    fn name(&self) -> String;

    // default function
    fn speak(&self) -> String {
        String::from("... makes a sound")
    }
}

struct Dog {
    name: String
}

struct Cat {
    name: String
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn speak(&self) -> String {
        format!("{} says Woof!", self.name())
    }
}

impl Dog {
    fn new(name: &str) -> Self {
        Dog { name: name.to_string() }
    }
}

impl Cat {
    fn new(name: &str) -> Self {
        Cat { name: name.to_string() }
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn speak(&self) -> String {
        format!("{} says Meow!", self.name())
    }
}

fn print_info<T: fmt::Display>(item: T) {
    println!("{}", item);
}

struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector2D({}, {})", self.x, self.y)
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    // Day 10: Traits 🦀

    // 1
    let dog = Dog::new("Buddy");
    let cat = Cat::new("Whiskers");
    println!("{}", dog.speak()); // Buddy says Woof!
    println!("{}", cat.speak()); // Whiskers says Meow!

    // 2
    print_info(dog.name()); // Buddy
    print_info(cat.name()); // Whiskers

    // 3
    let v1 = Vector2D::new(1.0, 2.0);
    let v2 = Vector2D::new(3.0, 4.0);
    let v3 = v1 + v2;
    println!("{}", v3); // Vector2D(4.0, 6.0)
}
