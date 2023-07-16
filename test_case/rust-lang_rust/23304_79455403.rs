
#[derive(Debug)]
enum Delicious {
    Pie      =  0x1,
    Apple    =  0x2,
    IceCream = 0x40,
    Vanilla  = 0x80,
    ApplePie = Delicious::Apple as isize | Delicious::Pie as isize,
    PieALaMode = Delicious::ApplePie as isize | Delicious::IceCream as isize | Delicious::Vanilla as isize,
}

fn main() {
    println!("Apple Pie = {:?} ({:?})", Delicious::ApplePie, Delicious::ApplePie as isize);
    println!("Pie à la Mode = {:?} ({:?})", Delicious::PieALaMode, Delicious::PieALaMode as isize);
}
