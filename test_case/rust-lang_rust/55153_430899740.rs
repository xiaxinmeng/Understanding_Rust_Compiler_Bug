rust
struct Sheep {
    message: String
}

impl Sheep {
    fn talk(self) {
       ^^^^^^^^^ ... mover
        println!("{}", self.message);
    }
}

fn main()
{
    let sheep = &Sheep { message: "Määh".into() };
    sheep.talk();
    ^^^^^ ... moved by `Sheep::talk`
}
