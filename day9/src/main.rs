use std::fmt::Display;

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

fn print_info<T: Display>(item: T) {
    println!("{}", item);
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
}
