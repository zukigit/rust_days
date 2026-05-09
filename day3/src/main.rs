fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn fizzbuzz(n: u32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn main() {
    let result = multiply(5, 3);
    println!("The result is: {}", result);

    fizzbuzz(20);

    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("The sum is: {}", sum);
}
