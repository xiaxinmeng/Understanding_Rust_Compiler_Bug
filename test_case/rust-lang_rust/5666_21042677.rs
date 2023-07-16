 rust
struct Dog {
    name : ~str
}

trait Barks {
    fn bark(&self) -> ~str;
}

impl Barks for Dog {
    fn bark(&self) -> ~str {
        return fmt!("woof! (I'm %s)", self.name);
    }
}


fn main() {
    let snoopy = ~Dog{name: ~"snoopy"};
    let bubbles = ~Dog{name: ~"bubbles"};
    let barker = [snoopy as ~Barks, bubbles as ~Barks];

    for barker.iter().advance |pup| {
        println(fmt!("%s", pup.bark()));
    }
}
