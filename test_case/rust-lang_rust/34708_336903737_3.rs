Rust
fn foo(x: Box<[Box<[Box<u32>]>]>) {
    match rand() {
        0 => match x {
            box [box [x, ..], ..] => use(x)
        },
        1 => match x {
            box [.., box [x, ..]] => use(x)
        },
        2 => match x {
            box [_, box [_, x, ..], ..] => use(x)
        }
        // etc.
    }
}
