
trait Shizzle {
    fn shizzle();
}

trait Shozzle : Shizzle {
    fn shizzle() {
        io::println(~"shizzle! " + self.shozzle());
    }
    fn shozzle() -> ~str;
}

impl int : Shozzle {
    fn shozzle() -> ~str { ~"i'm an int!" }
}

fn main() {
    2.shizzle();
}
