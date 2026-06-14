fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn parse_and_double(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num * 2),
        Err(e) => Err(format!("Failed to parse '{}' as an integer: {}", s, e)),
    }
}

fn parse_age(s: &str) -> Result<&str, String> {
    let age: i32 = s.parse().map_err(|e| format!("Failed to parse '{}' as an integer: {}", s, e))?;
    if age < 0 || age > 120 {
        Err(format!("Age {} is out of range (must be 0–120)", age))
    } else {
        Ok("valid")
    }
}

fn main() {
    // Day 8: Option & Result — Error Handling

    let result = safe_divide(10.0, 2.0);
    match result {
        Some(value) => println!("The result of division is {}.", value),
        None => println!("Cannot divide by zero!"),
    }

    let parse_result = parse_and_double("42");
    match parse_result {
        Ok(value) => println!("The doubled value is {}.", value),
        Err(e) => println!("Error: {}", e),
    }

    let age_result = parse_age("25");
    match age_result {
        Ok(msg) => println!("Age is {}.", msg),
        Err(e) => println!("Error: {}", e),
    }

    let age_result_invalid = parse_age("200");
    match age_result_invalid {
        Ok(msg) => println!("Age is {}.", msg),
        Err(e) => println!("Error: {}", e),
    }
}
