
% rustc plugin.rs
...
% rustc main.rs -L . --target arm-unknown-linux-gnueabi
main.rs:4:11: 4:31 error: can't find crate for `plugin`
main.rs:4 #[plugin] extern crate plugin;
                    ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
