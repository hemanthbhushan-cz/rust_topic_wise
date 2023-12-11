#[derive(Debug)]
struct Person<T: NotDangerous> {
    pet: T,
    pet_owner: String,
}
#[derive(Debug)]
struct Person2<T: Dangerous> {
    pet: T,
    pet_owner: String,
}

pub trait NotDangerous {
    fn pet_sound();
}
pub trait Dangerous {}
#[derive(Debug)]
struct Dog {}
impl NotDangerous for Dog {
    fn pet_sound() {
        println!("im the dog and i will bark");
    }
}
#[derive(Debug)]

struct Cat {}
impl NotDangerous for Cat {
    fn pet_sound() {
        println!("im the cat and i will mewww");
    }
}
#[derive(Debug)]
struct Tiger {}
impl Dangerous for Tiger {}

pub fn check() {
    let pet1 = Cat {};
    let p1 = Person {
        pet: pet1,
        pet_owner: "rfrf".to_string(),
    };

    let p2: Person2<Tiger> = Person2 {
        pet: Tiger {},
        pet_owner: "hemanth".to_string(),
    };

    println!("{:?}, {:?}", p1, p2)
}
