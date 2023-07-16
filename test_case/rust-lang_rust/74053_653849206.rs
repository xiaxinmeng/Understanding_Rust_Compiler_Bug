rust
struct Foo {
    val: i32,
}

const FOO: Foo = Foo { val: 100 };

const fn bar() -> i32 {
    FOO.val = -100;
    FOO.val
}

fn main() {
    println!("{}", FOO.val);
    println!("{}", bar());
}
