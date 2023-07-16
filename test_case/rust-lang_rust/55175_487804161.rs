rust
struct Foo(i32);

fn main() {
    let foo = Foo(5);
    
    match &foo {
        // To dereference and copy bar, match on the reference
        Foo(&bar) => {
            let _: i32 = bar;
        },
    }
}
