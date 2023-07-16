plain
travis_time:start:test_stage1-serialize
Testing serialize stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:04]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
[01:24:07] error[E0308]: mismatched types
[01:24:07]    --> libserialize/opaque.rs:370:44
[01:24:07]     |
[01:24:07] 370 |             let mut encoder = Encoder::new(&mut cursor);
[01:24:07]     |                                            ^^^^^^^^^^^ expected struct `std::vec::Vec`, found mutable reference
[01:24:07]     = note: expected type `std::vec::Vec<u8>`
[01:24:07]     = note: expected type `std::vec::Vec<u8>`
[01:24:07]                found type `&mut std::io::Cursor<std::vec::Vec<_>>`
[01:24:07] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:24:07]    --> libserialize/leb128.rs:163:37
[01:24:07]     |
[01:24:07]     |
[01:24:07] 36  |         pub fn $fn_name(out: &mut Vec<u8>, mut value: $int_ty) {
[01:24:07]     |         ------------------------------------------------------ defined here
[01:24:07] ...
[01:24:07] 163 |                 let bytes_written = $write_fn_name(&mut stream, pos, (3u64 << x) as $int_ty);
[01:24:07] ...
[01:24:07] ...
[01:24:07] 179 | impl_test_unsigned_leb128!(test_u16_leb128, write_u16_leb128, read_u16_leb128, u16);
[01:24:07]     | ------------------------------------------------------------------------------------ in this macro invocation
[01:24:07] 
[01:24:07] error[E0277]: cannot add `()` to `usize`
[01:24:07]    --> libserialize/leb128.rs:164:46
[01:24:07]     |
[01:24:07] 164 |                 assert_eq!(stream.len(), pos + bytes_written);
[01:24:07]     |                                              ^ no implementation for `usize + ()`
[01:24:07] ...
[01:24:07] 179 | impl_test_unsigned_leb128!(test_u16_leb128, write_u16_leb128, read_u16_leb128, u16);
[01:24:07]     | ------------------------------------------------------------------------------------ in this macro invocation
[01:24:07]     |
[01:24:07]     = help: the trait `std::ops::Add<()>` is not implemented for `usize`
[01:24:07] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:24:07]    --> libserialize/leb128.rs:163:37
[01:24:07]     |
[01:24:07]     |
[01:24:07] 36  |         pub fn $fn_name(out: &mut Vec<u8>, mut value: $int_ty) {
[01:24:07]     |         ------------------------------------------------------ defined here
[01:24:07] ...
[01:24:07] 163 |                 let bytes_written = $write_fn_name(&mut stream, pos, (3u64 << x) as $int_ty);
[01:24:07] ...
[01:24:07] ...
[01:24:07] 180 | impl_test_unsigned_leb128!(test_u32_leb128, write_u32_leb128, read_u32_leb128, u32);
[01:24:07]     | ------------------------------------------------------------------------------------ in this macro invocation
[01:24:07] 
[01:24:07] error[E0277]: cannot add `()` to `usize`
[01:24:07]    --> libserialize/leb128.rs:164:46
[01:24:07]     |
[01:24:07] 164 |                 assert_eq!(stream.len(), pos + bytes_written);
[01:24:07]     |                                              ^ no implementation for `usize + ()`
[01:24:07] ...
[01:24:07] 180 | impl_test_unsigned_leb128!(test_u32_leb128, write_u32_leb128, read_u32_leb128, u32);
[01:24:07]     | ------------------------------------------------------------------------------------ in this macro invocation
[01:24:07]     |
[01:24:07]     = help: the trait `std::ops::Add<()>` is not implemented for `usize`
[01:24:07] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:24:07]    --> libserialize/leb128.rs:163:37
[01:24:07]     |
[01:24:07]     |
[01:24:07] 36  |         pub fn $fn_name(out: &mut Vec<u8>, mut value: $int_ty) {
[01:24:07]     |         ------------------------------------------------------ defined here
[01:24:07] ...
[01:24:07] 163 |                 let bytes_written = $write_fn_name(&mut stream, pos, (3u64 << x) as $int_ty);
[01:24:07] ...
[01:24:07] ...
[01:24:07] 181 | impl_test_unsigned_leb128!(test_u64_leb128, write_u64_leb128, read_u64_leb128, u64);
[01:24:07]     | ------------------------------------------------------------------------------------ in this macro invocation
[01:24:07] 
[01:24:07] error[E0277]: cannot add `()` to `usize`
[01:24:07]    --> libserialize/leb128.rs:164:46
[01:24:07]     |
[01:24:07] 164 |                 assert_eq!(stream.len(), pos + bytes_written);
[01:24:07]     |                                              ^ no implementation for `usize + ()`
[01:24:07] ...
[01:24:07] 181 | impl_test_unsigned_leb128!(test_u64_leb128, write_u64_leb128, read_u64_leb128, u64);
[01:24:07]     | ------------------------------------------------------------------------------------ in this macro invocation
[01:24:07]     |
[01:24:07]     = help: the trait `std::ops::Add<()>` is not implemented for `usize`
[01:24:07] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:24:07]    --> libserialize/leb128.rs:163:37
[01:24:07]     |
[01:24:07]     |
[01:24:07] 36  |         pub fn $fn_name(out: &mut Vec<u8>, mut value: $int_ty) {
[01:24:07]     |         ------------------------------------------------------ defined here
[01:24:07] ...
[01:24:07] 163 |                 let bytes_written = $write_fn_name(&mut stream, pos, (3u64 << x) as $int_ty);
[01:24:07] ...
[01:24:07] ...
[01:24:07] 182 | impl_test_unsigned_leb128!(test_u128_leb128, write_u128_leb128, read_u128_leb128, u128);
[01:24:07]     | ---------------------------------------------------------------------------------------- in this macro invocation
[01:24:07] 
[01:24:07] error[E0277]: cannot add `()` to `usize`
[01:24:07]    --> libserialize/leb128.rs:164:46
[01:24:07]     |
[01:24:07] 164 |                 assert_eq!(stream.len(), pos + bytes_written);
[01:24:07]     |                                              ^ no implementation for `usize + ()`
[01:24:07] ...
[01:24:07] 182 | impl_test_unsigned_leb128!(test_u128_leb128, write_u128_leb128, read_u128_leb128, u128);
[01:24:07]     | ---------------------------------------------------------------------------------------- in this macro invocation
[01:24:07]     |
[01:24:07]     = help: the trait `std::ops::Add<()>` is not implemented for `usize`
[01:24:07] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:24:07]    --> libserialize/leb128.rs:163:37
[01:24:07]     |
[01:24:07]     |
[01:24:07] 36  |         pub fn $fn_name(out: &mut Vec<u8>, mut value: $int_ty) {
[01:24:07]     |         ------------------------------------------------------ defined here
[01:24:07] ...
[01:24:07] 163 |                 let bytes_written = $write_fn_name(&mut stream, pos, (3u64 << x) as $int_ty);
[01:24:07] ...
[01:24:07] ...
[01:24:07] 183 | impl_test_unsigned_leb128!(test_usize_leb128, write_usize_leb128, read_usize_leb128, usize);
[01:24:07]     | -------------------------------------------------------------------------------------------- in this macro invocation
[01:24:07] 
[01:24:07] error[E0277]: cannot add `()` to `usize`
[01:24:07]    --> libserialize/leb128.rs:164:46
[01:24:07]     |
[01:24:07] 164 |                 assert_eq!(stream.len(), pos + bytes_written);
[01:24:07]     |                                              ^ no implementation for `usize + ()`
[01:24:07] ...
[01:24:07] 183 | impl_test_unsigned_leb128!(test_usize_leb128, write_usize_leb128, read_usize_leb128, usize);
[01:24:07]     | -------------------------------------------------------------------------------------------- in this macro invocation
[01:24:07]     |
[01:24:07]     = help: the trait `std::ops::Add<()>` is not implemented for `usize`
[01:24:07] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[01:24:07]    --> libserialize/leb128.rs:191:29
[01:24:07]     |
[01:24:07]     |
[01:24:07] 125 | pub fn write_signed_leb128(out: &mut Vec<u8>, value: i128) {
[01:24:07]     | ---------------------------------------------------------- defined here
[01:24:07] ...
[01:24:07] 191 |         let bytes_written = write_signed_leb128(&mut stream, pos, x);
[01:24:07] 
[01:24:07] 
[01:24:07] error[E0277]: cannot add `()` to `usize`
[01:24:07]    --> libserialize/leb128.rs:192:38
[01:24:07]     |
[01:24:07] 192 |         assert_eq!(stream.len(), pos + bytes_written);
[01:24:07]     |                                      ^ no implementation for `usize + ()`
[01:24:07]     |
[01:24:07]     = help: the trait `std::ops::Add<()>` is not implemented for `usize`
[01:24:07] error: aborting due to 13 previous errors
[01:24:07] 
[01:24:07] Some errors occurred: E0061, E0277, E0308.
[01:24:07] For more information about an error, try `rustc --explain E0061`.
[01:24:07] For more information about an error, try `rustc --explain E0061`.
[01:24:07] error: Could not compile `serialize`.
[01:24:07] To learn more, run the command again with --verbose.
[01:24:07] 
[01:24:07] 
[01:24:07] 
[01:24:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "serialize" "--" "--quiet"
[01:24:07] 
[01:24:07] 
[01:24:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:07] Build completed unsuccessfully in 0:40:19
[01:24:07] Build completed unsuccessfully in 0:40:19
[01:24:07] make: *** [check] Error 1
[01:24:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06ad4040
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1cc51f00:start=1529509557928437505,finish=1529509557933735193,duration=5297688
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0812c578
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2dd5d1e2
$ dmesg | grep -i kill
