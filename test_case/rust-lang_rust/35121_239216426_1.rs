
impl Foo for MyStruct {
    type Bar = !;
    fn do_something(&self) -> ! { panic!() }
}
