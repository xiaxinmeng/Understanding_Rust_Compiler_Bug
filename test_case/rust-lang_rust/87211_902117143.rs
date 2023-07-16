
#[rustc_on_unimplemented(
    on(
        _Self = "&impl Future", 
        label = "immutable reference to a future does not implement a future",
    ), 
    label = "`{Self}` is not a future", message = "`{Self}` is not a future"
)]
