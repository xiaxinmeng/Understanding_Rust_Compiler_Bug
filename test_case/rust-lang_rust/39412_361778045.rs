rust
macro m_helper() {
    struct #S;
    struct T;
}

macro m() {
    lift!(m_helper!()); // Sets caller context of `m_helper!` to caller context of `m!`.
    let s = S; // Not OK: `S` is in callers context.
    let t = T; // Not OK: `T` is in `m_helper!` context.
}

fn main() {
    m!();
    let s = S; // OK: `S` is in `main` context.
}
