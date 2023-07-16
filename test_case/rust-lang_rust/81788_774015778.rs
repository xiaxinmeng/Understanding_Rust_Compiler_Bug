rust
fn test() -> () {
    let mut _0: ();                      // return place in scope 0 at src/test/ui/issues/issue-74614.rs:4:14: 4:14
    scope 1 (inlined std::mem::size_of::<T>) { // at src/test/ui/issues/issue-74614.rs:5:5: 5:29
    }

    bb0: {
        _0 = const ();                   // scope 0 at src/test/ui/issues/issue-74614.rs:4:14: 6:2
        return;                          // scope 0 at src/test/ui/issues/issue-74614.rs:6:2: 6:2
    }
}
