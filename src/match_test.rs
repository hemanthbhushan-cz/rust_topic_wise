pub fn match_test() {
    let number = 22;

    match number {
        1..=10 => println!("your kid"),
        11..=24 => println!("your old"),
        _ => println!("none"),
    }
}

pub fn match_test_array() {
    let number: [i32; 5] = [1, 2, 3, 4, 5];

    match number {
        [..] => println!("your kid"),
        [1, 3, 43, 4, 5] => println!("your old"),
        _ => println!("none"),
    }
}
