`rust
#[derive(Copy, Clone)]
enum Never {}

union Foo {
    a: u64,
    b: Never
}

fn main() {
    let _f = [Foo { a: 42 }];
}
