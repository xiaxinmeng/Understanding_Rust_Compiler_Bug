 rust
pub struct Event
{
    ...
    callback_fn: fn(ev: &Event),
    user_data: ~Any
}
