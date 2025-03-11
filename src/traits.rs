// Animal trait definition
pub trait Animal {
    fn sound(&self) -> String;
}

// Cat implementation
pub struct Cat;

impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Meow!")
    }
}

// Dog implementation
pub struct Dog;

impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("Woof!")
    }
}

// Orchestra implementation
pub struct Orchestra {
    animals: Vec<Box<dyn Animal>>
}

impl Orchestra {
    pub fn new() -> Self {
        Orchestra {
            animals: Vec::new()
        }
    }

    pub fn add_animal(&mut self, animal: Box<dyn Animal>) {
        self.animals.push(animal);
    }

    pub fn song(&self) -> String {
        if self.animals.is_empty() {
            return String::from("*silence*");
        }
        self.animals
            .iter()
            .map(|animal| animal.sound())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_sound() {
        let cat = Cat;
        assert_eq!(cat.sound(), "Meow!");
    }

    #[test]
    fn test_dog_sound() {
        let dog = Dog;
        assert_eq!(dog.sound(), "Woof!");
    }

    #[test]
    fn test_empty_orchestra() {
        let orchestra = Orchestra::new();
        assert_eq!(orchestra.song(), "*silence*");
    }

    #[test]
    fn test_orchestra_with_animals() {
        let mut orchestra = Orchestra::new();
        orchestra.add_animal(Box::new(Cat));
        orchestra.add_animal(Box::new(Dog));
        orchestra.add_animal(Box::new(Cat));
        assert_eq!(orchestra.song(), "Meow! Woof! Meow!");
    }
}
