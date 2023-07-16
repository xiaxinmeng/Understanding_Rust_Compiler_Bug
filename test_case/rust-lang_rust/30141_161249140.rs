 rust
enum Cake {
    BlackForest,
    Marmor,
}
use Cake::*;

const BOO: (Cake, Cake) = (Marmor, BlackForest);
const FOO: Cake = BOO.1;

fn main() {
    match BlackForest {
        FOO => println!("hi"),
        _ => println!("bye"),
    }
}
