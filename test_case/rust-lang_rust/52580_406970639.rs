plain
[01:04:12] 
[01:04:12]    Doc-tests core
[01:04:17] 
[01:04:17] running 2100 tests
[01:04:24] .....................................................................FF.............................
[01:04:32] ...........................F.FFF....................................................................
[01:04:51] .....................................i..............................................................
[01:04:57] ....................................................................................................
[01:05:04] ....................................................................................................
[01:05:11] ....................................................................................................
---
[01:06:57] 
[01:06:57] failures:
[01:06:57] 
[01:06:57] ---- char/methods.rs - char::methods::char::encode_utf16 (line 515) stdout ----
[01:06:57] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[01:06:57]  --> char/methods.rs:518:18
[01:06:57]   |
[01:06:57] 6 | let result = '𝕊'.try_encode_utf16(&mut b).unwrap();
[01:06:57]   |
[01:06:57]   |
[01:06:57]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[01:06:57] thread 'char/methods.rs - char::methods::char::encode_utf16 (line 515)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:06:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:57] 
[01:06:57] ---- char/methods.rs - char::methods::char::encode_utf16 (line 525) stdout ----
[01:06:57] ---- char/methods.rs - char::methods::char::encode_utf16 (line 525) stdout ----
[01:06:57] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[01:06:57]  --> char/methods.rs:528:22
[01:06:57]   |
[01:06:57] 6 | assert_eq!(None, '𝕊'.try_encode_utf16(&mut b));
[01:06:57]   |
[01:06:57]   |
[01:06:57]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[01:06:57] thread 'char/methods.rs - char::methods::char::encode_utf16 (line 525)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:06:57] 
[01:06:57] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 549) stdout ----
[01:06:57] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 549) stdout ----
[01:06:57] error[E0599]: no method named `unwrap` found for type `&mut [u16]` in the current scope
[01:06:57]  --> char/methods.rs:552:39
[01:06:57]   |
[01:06:57] 6 | let result = '𝕊'.encode_utf16(&mut b).unwrap();
[01:06:57] 
[01:06:57] thread 'char/methods.rs - char::methods::char::try_encode_utf16 (line 549)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:06:57] 
[01:06:57] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 467) stdout ----
[01:06:57] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 467) stdout ----
[01:06:57] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[01:06:57]  --> char/methods.rs:470:22
[01:06:57]   |
[01:06:57] 6 | assert_eq!(None, 'ß'.try_encode_utf8(&mut b));
[01:06:57]   |
[01:06:57]   |
[01:06:57]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[01:06:57] thread 'char/methods.rs - char::methods::char::try_encode_utf8 (line 467)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:06:57] 
[01:06:57] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 455) stdout ----
[01:06:57] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 455) stdout ----
[01:06:57] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[01:06:57]  --> char/methods.rs:458:18
[01:06:57]   |
[01:06:57] 6 | let result = 'ß'.try_encode_utf8(&mut b).unwrap();
[01:06:57]   |
[01:06:57]   |
[01:06:57]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[01:06:57] thread 'char/methods.rs -1 passed; 6 failed; 3 ignored; 0 measured; 0 filtered out
[01:06:57] 
[01:06:57] error: test failed, to rerun pass '--doc'
[01:06:57] 
[01:06:57] 
[01:06:57] 
[01:06:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:06:57] 
[01:06:57] 
[01:06:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:57] Build completed unsuccessfully in 0:22:02
[01:06:57] Build completed unsuccessfully in 0:22:02
[01:06:57] Makefile:58: recipe for target 'check' failed
[01:06:57] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03604264
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
