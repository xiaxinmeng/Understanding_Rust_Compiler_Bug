rust
fn main() {
    let a = 0;
    let gen = move || {
        drop(a);
        yield;
    };
    println!("{}", std::mem::size_of_val(&gen));
}
