plain
error[E0423]: expected value, found module `ptr`
    --> library/alloc/src/rc.rs:2791:22
     |
2791 |         let result = ptr.as_ptr();
     |                      ^^^ help: you might have meant to use the available field: `self.ptr`
For more information about this error, try `rustc --explain E0423`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:18
