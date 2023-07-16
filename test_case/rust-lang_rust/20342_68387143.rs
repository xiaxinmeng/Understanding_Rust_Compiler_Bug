
src/lib.rs:1:1: 1:1 error: multiple dylib candidates for `std` found
src/lib.rs:1 #![crate_name = "bindgen"]
             ^
src/lib.rs:1:1: 1:1 note: candidate #1: /usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so
src/lib.rs:1 #![crate_name = "bindgen"]
             ^
src/lib.rs:1:1: 1:1 note: candidate #2: /usr/local/lib/libstd-4e7c5e5c.so
src/lib.rs:1 #![crate_name = "bindgen"]
             ^
src/lib.rs:8:24: 8:41 error: multiple dylib candidates for `log` found
src/lib.rs:8 #[phase(plugin, link)] extern crate log;
                                    ^~~~~~~~~~~~~~~~~
src/lib.rs:8:24: 8:41 note: candidate #1: /usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-4e7c5e5c.so
src/lib.rs:8 #[phase(plugin, link)] extern crate log;
                                    ^~~~~~~~~~~~~~~~~
src/lib.rs:8:24: 8:41 note: candidate #2: /usr/local/lib/liblog-4e7c5e5c.so
src/lib.rs:8 #[phase(plugin, link)] extern crate log;
                                    ^~~~~~~~~~~~~~~~~
src/lib.rs:8:24: 8:41 error: multiple dylib candidates for `regex` found
src/lib.rs:8 #[phase(plugin, link)] extern crate log;
                                    ^~~~~~~~~~~~~~~~~
src/lib.rs:8:24: 8:41 note: candidate #1: /usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libregex-4e7c5e5c.so
src/lib.rs:8 #[phase(plugin, link)] extern crate log;
                                    ^~~~~~~~~~~~~~~~~
src/lib.rs:8:24: 8:41 note: candidate #2: /usr/local/lib/libregex-4e7c5e5c.so
src/lib.rs:8 #[phase(plugin, link)] extern crate log;
