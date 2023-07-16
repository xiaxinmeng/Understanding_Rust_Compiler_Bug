rust
macro m_helper() {
    struct #S;
}

macro m() {
    m_helper!(); // `S` has this context
    let s = S; // OK
}

fn main() {
    m!(); // `S` is not accessible here
    let s = S; // ERROR
}
