rust
#[unwind]
extern "C" fn banana(x: *const ()) -> *const () {
    ::std::thread::sleep(::std::time::Duration::from_millis(500)); // might unwind due to pthread_cancel and is not UB anymore, because `banana` is specified to unwind.
    x
}
