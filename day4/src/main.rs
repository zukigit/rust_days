fn first_word(s: &String) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return i;
        }
    }

    return s.len()
}

fn longer_length(s1: &String, s2: &String) -> usize {
    if s1.len() > s2.len() {
        return s1.len();
    }

    return s2.len();
}

fn main() {
    let index_of_first_space = first_word(&String::from("hello world"));
    println!("index of first space: {}", index_of_first_space);

    let mut my_string = String::from("hello world");

    let ref_1 = &mut my_string; // no problem
    let ref_2 = &mut my_string; // problem: cannot borrow `my_string` as mutable more than once at a time

    println!("{} {}", ref_1, ref_2);

    longer_length(&String::from("hello"), &String::from("world"));
}
