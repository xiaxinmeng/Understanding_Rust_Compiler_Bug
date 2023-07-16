
struct Foo { val: i32 }

fn main() {
    let mut owned_foo = Foo { val: 0 };
    let mut owned_foo2 = Foo { val: 1 };

    let mut foo_ref = &mut owned_foo;

    //This makes a mutable borrow not only of
    //`owned_foo`, but also of `foo_ref` *itself*...
    let val_ref = &mut foo_ref.val;

    //...so we can't repoint foo_ref to something
    //else here, even though it would be safe (the
    //memory val_ref points to is still backed by
    //owned_foo)
    foo_ref = &mut owned_foo2;
}
