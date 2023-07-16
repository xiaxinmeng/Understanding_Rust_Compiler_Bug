rust
#![feature(existential_type)]

existential type Foo: Copy;

// make compiler happy about using 'Foo'
fn bar(x: Foo) -> Foo { x }

fn main() {
    let x: Foo;
    let _: Foo = unsafe { std::mem::transmute(x) };
}
