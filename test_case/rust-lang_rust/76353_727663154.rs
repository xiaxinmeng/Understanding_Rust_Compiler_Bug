rust
struct Bar<'a>(&'a str);

impl<'a> Bar<'a> {
    fn hello(&self, name: &'a str) {
        println!("Hello {}!", name);
    }
    
    fn say(&self, f: fn(&Self, &str), name: &str) {
        f(self, name)
    }
    
}

fn main() {
    Bar("").say(Bar::hello, "World");
}
