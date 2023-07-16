
error: use of unstable library feature 'repeat_str' (see issue #0)
    --> src/libcollections/../libcollectionstest/str.rs:1291:19
     |
1291 |     assert_eq!("".repeat(3), "");
     |                   ^^^^^^
     |
     = help: add #![feature(repeat_str)] to the crate attributes to enable

error: use of unstable library feature 'repeat_str' (see issue #0)
    --> src/libcollections/../libcollectionstest/str.rs:1292:22
     |
1292 |     assert_eq!("abc".repeat(0), "");
     |                      ^^^^^^
     |
     = help: add #![feature(repeat_str)] to the crate attributes to enable

error: use of unstable library feature 'repeat_str' (see issue #0)
    --> src/libcollections/../libcollectionstest/str.rs:1293:20
     |
1293 |     assert_eq!("α".repeat(3), "ααα");
     |                    ^^^^^^
     |
     = help: add #![feature(repeat_str)] to the crate attributes to enable

error: aborting due to 3 previous errors

Build failed, waiting for other jobs to finish...
error: Could not compile `collections`.
