 rust
pub struct Window<'a> {
    callbacks: WindowCallbacks<'a>
}
struct WindowCallbacks<'a> {
    pos_callback:                Option<WindowPosCallback<'a>>,
}
pub type WindowPosCallback<'a> = 'a |&Window, i32, i32|;
