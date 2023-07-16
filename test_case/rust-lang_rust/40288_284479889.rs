rust
fn prove_static(_: [&'static str; 1]) {}

fn main() {
    let mut out = ["foo"];
    {
        let mut x = String::from("bar");
        let slice: &mut [_] = &mut out;
        slice[0] = &x[..];
    }
    let _new = String::from("boo");
    println!("{}", out[0]);
    prove_static(out);
}
