rust
const bar: u8 = 10;

fn main() {
    fn bar() {} // fn bar shadows const bar
    let bar = (); // OK, bar resolves to fn, fns can't be patterns
}
