
struct S;

enum Option<T> {
    Some(T),
    None,
}

let s = S;
match s {
    S => {}
}

let opt = Some(0u8);
match opt {
    None => {}
    _ => {}
}
