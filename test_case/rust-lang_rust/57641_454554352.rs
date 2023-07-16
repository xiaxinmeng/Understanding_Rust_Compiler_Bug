rust
#[my_custom_macro] // this can read and modify the module declaration and everything in it
mod foo {
    ...
}

my_custom_macro! {
    // this can only modify the stuff in the block here
    ...
}
