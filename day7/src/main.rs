enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    fn is_warm(&self) -> bool {
        match self {
            Season::Spring | Season::Summer => true,
            Season::Autumn | Season::Winter => false,
        }
    }
}

fn main() {
    // Day 7: Enums & Pattern Matching 🦀Day 7: Enums & Pattern Matching

    let coin = Coin::Penny;
    println!("The value of the coin is {} cents.", coin.value_in_cents());

    let result = divide(10.0, 0.0);
    match result {
        Some(value) => println!("The result of division is {}.", value),
        None => println!("Cannot divide by zero!"),
    }

    let current_season = Season::Summer;
    println!("current season: {:?} is warm: {}", current_season, current_season.is_warm());
}
