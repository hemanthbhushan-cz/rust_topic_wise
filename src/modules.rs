use std::io;
pub fn check() {
    println!("check");
}

pub mod checkers {
    use std::io;

    pub fn check_for_input() {
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("error");

        println!("the enterd input is {}", name);
    }
}
