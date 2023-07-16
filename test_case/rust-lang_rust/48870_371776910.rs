rust
macro foo($t: ident) {
    macro $t() {
        foo!(moo);
        println!("Hello stackoverflow");
    }
}

fn main() {
    foo!(bar);
    bar!();
    moo!(); // error: cannot find macro `moo!` in this scope
}
