fn main() {
    let my_age = 25;
    let my_age = 25;

    println!("My age is: {}", my_age);

    let my_tuple = ("zuki", 25, true);
    println!("my_name: {}", my_tuple.0);
    println!("my_age: {}", my_tuple.1);
    println!("my_is_student: {}", my_tuple.2);

    let my_array = [1, 2, 3, 4, 5];
    println!("First element: {}", my_array[0]);
    println!("Last element: {}", my_array[my_array.len() - 1])
}