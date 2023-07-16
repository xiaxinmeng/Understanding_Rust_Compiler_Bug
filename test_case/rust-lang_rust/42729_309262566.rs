rust
struct MyObject {
    text: String,
    int: u32,
}

fn do_something_owned(_o: MyObject) {}

fn main() {
    let mut my_object = MyObject { text: "1".into(), int: 2 };
    do_something_owned(my_object);
    my_object.text = "3".into();
    println!("{:?}", my_object.int);
    //~^ ERROR: use of moved value: `my_object.int`
}
