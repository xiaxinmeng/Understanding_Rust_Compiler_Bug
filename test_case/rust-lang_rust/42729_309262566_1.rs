rust
struct MyObject {
    text: String,
}

impl Drop for MyObject {
    fn drop(&mut self) {}
}

fn do_something_owned(_o: MyObject) {}

fn main() {
    let mut my_object = MyObject { text: "1".into() };
    do_something_owned(my_object);
    my_object.text = "3".into();
    //~^ ERROR: partial reinitialization of uninitialized structure `my_object`
}
