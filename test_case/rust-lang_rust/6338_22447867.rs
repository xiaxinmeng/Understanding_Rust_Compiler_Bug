
fn main() {
    match (1, 2, 3) {
        (1, a, b) | (2, b, a) => {
                printfln!(a);
                printfln!(b);
            },
            _ => fail!(),
    }
}
