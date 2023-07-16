rust
fn main() {
    // my_object becomes initialized
    // ->
    // my_object.text becomes initialized
    let mut my_object = MyObject { text: String::from("sometest") };

    // my_object becomes uninitialized
    // ->
    // my_object.text becomes uninitialized
    do_something_owned(my_object);

    // my_object.text becomes initialized
    // ->
    // my_object becomes initialized (UNIMPLEMENTED)
    my_object.text = String::from("we're done");
}
