pub fn test_clousers() {
    let result: i32 = 2;
    let clouser = || println!("print the input {}", result);
    println!("clouser {:?}", clouser());
    let clouser = |x| result + x;
    println!("clouser {:?}", clouser(23));
}
#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
}

pub fn test_clousers_for_mutability() {
    let mut p1 = Person {
        age: 23,
        name: "hemanth".to_string(),
    };

    let mut clouser = |age| {
        p1.age = age;
    };
    clouser(27);

    println!("check the pi after change in the clouser {:?}", p1);
}
