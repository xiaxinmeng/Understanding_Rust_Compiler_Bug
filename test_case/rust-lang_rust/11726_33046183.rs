 rust
use std::local_data;
fn foo() {
    local_data_key!(foo: uint);
    local_data::get(foo, |val| { ... });
    local_data::set(foo, 4);
}
