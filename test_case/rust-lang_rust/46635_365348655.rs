rust
// `pp!` expands to `quote!($input { my_field: 0 })` without a `#[uses(my_field)]`

mod m {
    pub struct MyStruct {
        my_field: i32,
    }
    const INNER: MyStruct = pp!(MyStruct); // ok, field is visible
}

const OUTER: m::MyStruct = pp!(m::MyStruct); // fails to compile
