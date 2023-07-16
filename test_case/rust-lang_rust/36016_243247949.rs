
union U {
    a: u8,
    b: u8,
}

const C: U = U { a: 10 }; // <-- here we hide union expression in a constant

fn main() {
    match C {
        C => {} // <-- here we are trying to match on that constant
        _ => {}
    }
}
