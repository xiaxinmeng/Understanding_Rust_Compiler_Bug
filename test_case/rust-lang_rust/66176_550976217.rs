rust
struct MyStruct<'a> {
    val: &'a bool
}

impl<'a> MyStruct<'a> {
    fn make_new() {
        let new_val = true;
        let my_val = MyStruct::<'a> { val: &new_val };
        //~^ ERROR `new_val` does not live long enough
    }
}
