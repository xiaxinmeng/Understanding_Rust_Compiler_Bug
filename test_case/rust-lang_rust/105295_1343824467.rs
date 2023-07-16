rust
trait Trait {
    fn get<'s>(s: &'s str, _: &'static &'static ()) -> &'static str;
}
impl Trait for () {
    fn get<'s>(s: &'s str, _: &'static &'s ()) -> &'static str {
        s
    }
}
fn main() {
    let val = <() as Trait>::get(&String::from("blah blah blah"), &&());
    println!("{}", val);
}
