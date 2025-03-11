mod traits;
mod errors;

use traits::{Animal, Orchestra, Cat, Dog};
use errors::read_numbers_from_file;

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

fn error_example() {
    println!("\nError handling example:");
    
    // Try reading from a non-existent file
    match read_numbers_from_file("nonexistent.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Create a file with invalid content
    std::fs::write("test.txt", "1\nnot_a_number\n3").unwrap();
    match read_numbers_from_file("test.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Create a file with valid content
    std::fs::write("test.txt", "1\n2\n3").unwrap();
    match read_numbers_from_file("test.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Clean up
    let _ = std::fs::remove_file("test.txt");
}

fn main() {
    trait_example();
    error_example();
}
