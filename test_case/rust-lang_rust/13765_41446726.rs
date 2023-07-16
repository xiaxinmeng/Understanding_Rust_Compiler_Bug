
fn main() {
    let mut foo = "abcd".chars();
    let bar = foo.by_ref().take_while(|&x| x < 'c').collect::<~str>();
    let baz = foo.collect::<~str>();
    println!("{} {}", bar, baz);
}
