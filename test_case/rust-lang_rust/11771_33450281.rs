
fn main() {
    let x = ();
    // Error prints correctly
    1 + x;

    let x: () = ();
    // Error prints backwards
    1 + x;
}
