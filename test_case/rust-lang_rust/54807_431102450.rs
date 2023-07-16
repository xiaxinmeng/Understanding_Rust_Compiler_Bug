rust
enum E {A, B, C}

fn foo(e: E) -> u32 {
    match e {
        E::A,E::B => 0, 
        E::C => 1
    }
}
