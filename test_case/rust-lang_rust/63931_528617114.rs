rust
#[my_attr]
mod m {
    mod n; // Out-of-line module, how should `my_attr` see it in its input?
}
