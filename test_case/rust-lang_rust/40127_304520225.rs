rust
use std::fs::read_dir;

fn foo() {
    let tests: Vec<_> = read_dir("tests/fixtures/").unwrap()
        .filter(|x| x.unwrap().path().is_dir())
        .collect();
}
