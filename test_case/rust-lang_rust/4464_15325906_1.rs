
4464.rs:1:55: 1:56 error: illegal borrow: borrowed value does not live long enough
4464.rs:1 fn broken(v: &'r [u8], i: uint, j: uint) -> &'r [u8] { v.slice(i, j) }
                                                                 ^
4464.rs:1:53: 1:70 note: borrowed pointer must be valid for the lifetime &'r  as defined on the block at 1:53...
4464.rs:1 fn broken(v: &'r [u8], i: uint, j: uint) -> &'r [u8] { v.slice(i, j) }
                                                               ^~~~~~~~~~~~~~~~~
4464.rs:1:0: 1:70 note: ...but borrowed value is only valid for the function body at 1:0
4464.rs:1 fn broken(v: &'r [u8], i: uint, j: uint) -> &'r [u8] { v.slice(i, j) }
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
