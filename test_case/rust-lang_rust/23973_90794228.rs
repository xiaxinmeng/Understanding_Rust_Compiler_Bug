 zsh
[/tmp] rustc snappy.rs
snappy.rs:1:1: 1:19 error: use of unstable library feature 'libc'
snappy.rs:1 extern crate libc;
            ^~~~~~~~~~~~~~~~~~
snappy.rs:1:19: 1:19 help: add #![feature(libc)] to the crate attributes to enable
snappy.rs:2:5: 2:17 error: use of unstable library feature 'libc'
snappy.rs:2 use libc::size_t;
                ^~~~~~~~~~~~
snappy.rs:2:17: 2:17 help: add #![feature(libc)] to the crate attributes to enable
snappy.rs:6:56: 6:62 error: use of unstable library feature 'libc'
snappy.rs:6         fn snappy_max_compressed_length(source_length: size_t) -> size_t;
                                                                   ^~~~~~
snappy.rs:6:62: 6:62 help: add #![feature(libc)] to the crate attributes to enable
snappy.rs:6:67: 6:73 error: use of unstable library feature 'libc'
snappy.rs:6         fn snappy_max_compressed_length(source_length: size_t) -> size_t;
                                                                              ^~~~~~
snappy.rs:6:73: 6:73 help: add #![feature(libc)] to the crate attributes to enable
error: aborting due to 4 previous errors
[/tmp] rustc snappy.rs
snappy.rs:1:1: 1:18 error: unstable feature
snappy.rs:1 #![feature(libc)]
            ^~~~~~~~~~~~~~~~~
note: this feature may not be used in the beta release channel
error: aborting due to previous error
