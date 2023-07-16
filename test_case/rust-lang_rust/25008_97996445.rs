 rust
fn main() {
    #[derive(Debug)]
    struct Recur<'a>(u8, Option<&'a Recur<'a>>);
    let r1 = Recur(2, None);
    let r2 = Recur(1, Some(&r1));
    match r2 {
        ref y @ Recur(1, _) | Recur(_, Some(&ref y)) => {
            println!("{:?}", y);
        }
        _ => { println!("other"); }
    }
}
