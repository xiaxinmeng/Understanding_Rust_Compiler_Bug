rust
struct MyBox<T>(ptr::NonNull<T>);

/// Mirror `Box` API, including `Drop`.
impl<T> Drop for MyBox<T> {
    fn drop (self: &'_ mut Self)
    {
        // ...
    }
}

fn main ()
{
    let (my_box, s): (MyBox<&'_ String>, String);
    s = String::from("hi");
    let at_s = &s;
    let at_s_ptr = ptr::NonNull::from(Box::leak(Box::new(at_s)));
    my_box = MyBox(at_s_ptr);
}
