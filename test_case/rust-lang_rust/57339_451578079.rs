rust
use pin_utils::pin_mut;

fn main() {
    let x = Foo { ... };
    pin_mut!(x);
    x.set(Foo { ... })
    // Pin<&mut x> does not implement Copy because &mut _ is (obviously) not copyable
    // So x has been consumed and there is no way to access the underlying value anymore...
}
