rust
fn foo<T>() {
    if some_const_boolean_property::<T>() {
        /* do nothing */
    } else {
        other_method::<T>();
    }
}
