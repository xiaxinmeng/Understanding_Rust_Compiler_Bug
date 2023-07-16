
fn main() {
    type u8 = ();
    let _x: u8 = 0; // this is allowed; "let _x = ();" is forbidden.
}
