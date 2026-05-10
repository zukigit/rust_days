fn last_word(s: &str) -> &str {
    s.split_whitespace().last().unwrap_or("")
}

fn largest_number(slice: &[i32]) -> i32 {
    slice.iter().max().cloned().unwrap_or(0)
}

fn main() {
    //Day 5: Slices & Strings
    println!("Last word: {}", last_word("Hello, world!"));

    let my_string = String::from("rust:is:awesome");
    let my_split_string = my_string.split(":");

    for word in my_split_string {
        println!("{}", word)
    }

    println!("Largest number: {}", largest_number(&[1, 5, 3, 9, 2]));
}
