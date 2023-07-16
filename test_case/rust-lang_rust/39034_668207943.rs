rust
use helpers::Animal;

#[forbid(dead_code)]
mod helpers {
    #[repr(u32)]
    pub enum Animal {
        Cat = 0, // Used: constructed
        Dog = 1, // Used: as pattern
        Frog = 2 // Error: variant is never used
    }
}

fn main() {
    let animal = Animal::Cat;

    match animal {
		Animal::Dog => println!("Woof!"),
		_ => println!("Something else.")
	}
}
