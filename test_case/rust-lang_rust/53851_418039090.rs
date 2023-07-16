
Sep 02 20:49:06.306 INFO kablam! error[E0597]: borrowed value does not live long enough
Sep 02 20:49:06.306 INFO kablam!    --> src/apint/constructors.rs:429:11
Sep 02 20:49:06.306 INFO kablam!     |
Sep 02 20:49:06.307 INFO kablam! 429 |               .chain([
Sep 02 20:49:06.307 INFO kablam!     |  ____________________^
Sep 02 20:49:06.307 INFO kablam! 430 | |                 u8::max_value(),
Sep 02 20:49:06.307 INFO kablam! 431 | |                 10,
Sep 02 20:49:06.307 INFO kablam! 432 | |                 42,
Sep 02 20:49:06.307 INFO kablam! 433 | |                 99,
Sep 02 20:49:06.307 INFO kablam! 434 | |                 123
Sep 02 20:49:06.307 INFO kablam! 435 | |             ].into_iter()
Sep 02 20:49:06.307 INFO kablam!     | |_____________^ temporary value does not live long enough
Sep 02 20:49:06.307 INFO kablam! 436 |                .map(|v| *v))
Sep 02 20:49:06.307 INFO kablam! 437 |       }
Sep 02 20:49:06.307 INFO kablam!     |       - temporary value only lives until here
Sep 02 20:49:06.307 INFO kablam!     |
Sep 02 20:49:06.307 INFO kablam!     = note: borrowed value must be valid for the static lifetime...
