 rust
trait Foo {
    fn foo(&self) -> u32 {
        1
    }
}

type Bar<F: Foo> = F;

fn foo(f: Bar<&str>) -> Bar<&str> {
    f
}

fn main() {
    let bar: Bar<&str> = "";
    println!("{:?}", foo(bar.foo()));
}
