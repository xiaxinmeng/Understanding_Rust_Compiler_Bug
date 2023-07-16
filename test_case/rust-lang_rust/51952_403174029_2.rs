rust
mod __implementation_details {

    use super::*; // <---

    // Now `MyStruct` and `SomethingElseFromTheParentModule ` are in scope.
    impl Something for MyStruct {
       let x: SomethingElseFromTheParentModule = ...;
    }
}
