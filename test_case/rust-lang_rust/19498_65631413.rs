
mod top {
    use top::submodule;  // "error: unresolved import use top::submodule;"
    mod submodule {}
}

fn main() {}
