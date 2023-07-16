rust
struct MyStruct;

mod __implementation_details {
    // Note, that `MyStruct` is not in scope here.
    impl Something for MyStruct {
        let x: SomethingElseFromTheParentModule = ...;
    }
}
