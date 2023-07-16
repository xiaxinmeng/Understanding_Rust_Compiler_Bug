
../src/libstd\sys/windows\compat.rs:31:39: 31:50 error: use of deprecated item: renamed to encode_utf16, #[deny(deprecated)] on by default
../src/libstd\sys/windows\compat.rs:31     let mut module: Vec<u16> = module.utf16_units().collect();
                                                                             ^~~~~~~~~~~
../src/libstd\sys/windows\stdio.rs:59:28: 59:39 error: use of deprecated item: renamed to encode_utf16, #[deny(deprecated)] on by default
../src/libstd\sys/windows\stdio.rs:59         Some(utf8) => utf8.utf16_units().collect::<Vec<u16>>(),
                                                                 ^~~~~~~~~~~
error: aborting due to 2 previous errors
