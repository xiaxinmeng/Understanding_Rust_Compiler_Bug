 shell
$ rustc main.rs
main.rs:6:5: 6:34 error: unresolved import `std::old_io::process::Command`. Could not find `old_io` in `std`
main.rs:6 use std::old_io::process::Command;
              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
main.rs:7:19: 7:23 error: unresolved import `std::old_io::File`. Could not find `old_io` in `std`
main.rs:7 use std::old_io::{File, TempDir};
                            ^~~~
main.rs:7:25: 7:32 error: unresolved import `std::old_io::TempDir`. Could not find `old_io` in `std`
main.rs:7 use std::old_io::{File, TempDir};
                                  ^~~~~~~
main.rs:8:5: 8:20 error: unresolved import `std::old_io::fs`. Could not find `old_io` in `std`
main.rs:8 use std::old_io::fs;
              ^~~~~~~~~~~~~~~
main.rs:9:5: 9:36 error: unresolved import `std::old_io::fs::PathExtensions`. Could not find `old_io` in `std`
main.rs:9 use std::old_io::fs::PathExtensions;
              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
main.rs:192:9: 192:24 error: unresolved import `std::old_io::fs`. Could not find `old_io` in `std`
main.rs:192     use std::old_io::fs;
                    ^~~~~~~~~~~~~~~
error: aborting due to 6 previous errors
