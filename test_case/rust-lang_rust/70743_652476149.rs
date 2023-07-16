rust
const A: &[u8; 1] = b".";
    match b".." as &[u8] {
        A => {},
        _ => panic!(),
    }
