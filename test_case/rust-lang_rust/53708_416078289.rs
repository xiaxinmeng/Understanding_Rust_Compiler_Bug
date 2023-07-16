rust
fn main() {
    const C: &bool = &true;
    let x = match C {
        C => "C",
        &true => "true",
        &false => "false",
    };
    println!("{}", x);
}
