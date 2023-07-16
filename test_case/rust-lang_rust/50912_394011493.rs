rust
fn matcher(x: u8, cond: bool) {
    match x {
        0..42 => {}
        43..255 if cond => {}
    }

    match x {
        0..42 => {}
        43..255 if x % 2 == 0 => {}
    }

    match x {
        0..42 => {}
        y @ 43..255 if y % 2 == 0 => {}
    }

    match x {
        0..42 => {}
        y @ 43..255 if y % 2 == 0 => {}
        43..255 => {}
    }
}
