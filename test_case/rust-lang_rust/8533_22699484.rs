 rust
fn main() {
    match (1,2,3) {
        (0, a, _) if a == 3 => {
            fail!();
        }
        (1, _, a) => {
            printfln!(a);
        }
        _ => {fail!()} // fails here
    }
}
