rust
#[rustc_on_unimplemented(
    on(
        Self="&str",
        label="`{Self}` is not an iterator; try calling `.chars()` or `.bytes()`"
    ),
    label="`{Self}` is not an iterator; maybe try calling `.iter()` or a similar method"
)]
