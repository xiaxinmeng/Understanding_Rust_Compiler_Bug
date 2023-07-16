rust
enum MyEnum {}

impl MyEnum {
    fn do_something(result: impl FnOnce()) {
        result();
    }

    fn do_something_extra() {
        fn inner() {
            Self::do_something(move || {}); // can't use generic parameters from outer function E0401
            MyEnum::do_something(move || {}); // no error
        }
        inner();
    }
}
