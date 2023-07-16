 rust
macro_rules! build_window_ffi(
    ($name:ident, $raw:ident, $ret:ty, $($arg_name:ident: $arg_ty:ty => $raw_arg:expr),*) => (
        impl Window {
            pub fn $name(&self, $($arg_name: $arg_ty),+) -> $ret {
               unsafe { ffi::$raw(self.handle, $($raw_arg),+) }
            }
        }
    )
)

build_window_ffi(DoSomething, SDL_DoSomething, ())
build_window_ffi(DoSomethingElse, SDL_DoSomethingElse, c_int)
