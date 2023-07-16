rust
fn program(x: u8) -> u8 {
    match x {
        1 | 2 => 2,
		3 | 4 => 0,
        _ => 1,
    }
}
