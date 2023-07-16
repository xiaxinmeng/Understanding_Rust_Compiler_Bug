 rust
export foo;
iface local_data { }
impl <T> of local_data for @T { }

fn foo<T>(t: @T) -> local_data {
    t as local_data
}
