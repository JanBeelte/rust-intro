fn wanna_own_me(s: String) {
    println!("Its '{}' now", s);
}

fn borrow_me(s: &str) {
    println!("I'm looking at your '{}'", s);
}

pub fn ownership_example() {
    println!("\nOwning and borrowing examples:");

    let s = "Stuff".to_string();
    borrow_me(&s);
    println!("I'm {}", s);

    wanna_own_me(s);
}
