
/home/tshepang/tmp/play/src/main.rs:6:24: 6:35 error: mismatched types: expected `*mut u8` but found `*const <generic #2>` (values differ in mutability)
/home/tshepang/tmp/play/src/main.rs:6     let nil: *mut u8 = ptr::null();
                                                             ^~~~~~~~~~~
/home/tshepang/tmp/play/src/main.rs:9:15: 9:18 error: mismatched types: expected `*const <generic #3>` but found `*mut u8` (values differ in mutability)
/home/tshepang/tmp/play/src/main.rs:9         data: nil,
                                                    ^~~
error: aborting due to 2 previous errors
Could not compile `play`.
