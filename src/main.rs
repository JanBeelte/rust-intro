mod traits;
use traits::{Animal, Orchestra, Cat, Dog};

fn trait_example() {

    // Individual animals
    let cat = Cat;
    let dog = Dog;
    
    println!("Individual performances:");
    println!("The cat says: {}", cat.sound());
    println!("The dog says: {}", dog.sound());

    // Orchestra performance
    println!("\nOrchestra performance:");
    let mut orchestra = Orchestra::new();
    orchestra.add_animal(Box::new(Cat));
    orchestra.add_animal(Box::new(Dog));
    orchestra.add_animal(Box::new(Cat));
    println!("The orchestra plays: {}", orchestra.song());
}

fn main() {
    trait_example();
}
