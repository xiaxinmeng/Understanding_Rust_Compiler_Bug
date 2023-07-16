rust
pub extern fn write(value: u16) -> u16 {
    let zoo = match value {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 6,
        6 => 7,
        7 => 229,
        _ => value + 1,
    };

    zoo
}
