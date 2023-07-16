rust
struct Foo;
fn mut_ref<'a, 'b>(val: &'a mut &'b mut Foo)  {
    let tmp: &'b mut Foo = *val;
}
