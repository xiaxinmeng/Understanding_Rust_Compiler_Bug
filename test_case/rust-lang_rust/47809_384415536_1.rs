rust
    #[rustc_track_caller]
    #[inline]
    #[...attributes...]
    fn foo(...) -> T {
        let __closure = |__location| { /* code... */ };
        FnOnce::call_once(__closure, intrinsics::caller_location())
    }
    