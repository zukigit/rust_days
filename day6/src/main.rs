#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    // area
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // circumference
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn new(owner: String, balance: f64) -> Self {
        BankAccount { owner, balance }
    }

    // deposit
    fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount < 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }

        self.balance += amount;
        Ok(self.balance)
    }

    // withdraw
    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount < 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }

        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }

        self.balance -= amount;
        Ok(self.balance)
    }

    // print statement
    fn print_statement(&self) {
        println!("Owner: {}", self.owner);
        println!("Balance: {}", self.balance);
    }
}

fn main() {
    // Day 6: Structs

    // 1
    let circle = Circle::new(5.0);
    println!("Circle with radius: {}", circle.radius);
    println!("Area: {}", circle.area());
    println!("Circumference: {}", circle.circumference());

    // 2
    let mut my_account = BankAccount::new("zuki".to_string(), 100000.0);
    my_account.print_statement();

    if let Err(e) = my_account.deposit(50000.0) {
        println!("Error: {}", e);
    }
    my_account.print_statement();

    if let Err(e) = my_account.withdraw(20000.0) {
        println!("Error: {}", e);
    }
    my_account.print_statement();

    // 3
    println!("{:#?}", circle);
    println!("{:#?}", my_account);
}
