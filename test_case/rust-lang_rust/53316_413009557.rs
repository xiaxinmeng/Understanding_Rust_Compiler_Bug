rust
#![feature(existential_type)]

existential type Foo: Copy;

// make compiler happy about using 'Foo'
fn bar(x: Foo) -> Foo { x }

fn main() {
    
    // make compiler happy about the types??
    let _: Foo = unsafe { std::mem::transmute(0u8) };
}
