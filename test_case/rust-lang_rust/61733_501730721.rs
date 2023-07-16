rust
fn main() {
    { () } // trailing expression

    #[cfg(FALSE)]
    0; // not trailing expression
}

fn main() {
    { () } // trailing expression

    #[cfg(FALSE)]
    0 // not trailing expression
}

fn main() {
    { () } // not trailing expression

    #[cfg(TRUE)]
    0; // not trailing expression
}

fn main() {
    { () } // not trailing expression

    #[cfg(TRUE)]
    0 // trailing expression
}
