
1 | fn closure_ret_closure<T: FnOnce() -> T>(f: T) -> T {
  |                                          ^^^^-----^ `closure_ret_closure` declares `f` and the return type to be the same
