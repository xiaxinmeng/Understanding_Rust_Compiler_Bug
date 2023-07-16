 rust
type Test<'a> = &'a mut Fn(&'static str) -> u32;

fn matched_lifetimes<'a>(_: &'a u8, _: Test<'a>) {}

fn main() {
    let x: Test<'static> = panic!();
    {
        let y: u8 = 0;
        let z = &y;
        matched_lifetimes(z, x);
    }
}
