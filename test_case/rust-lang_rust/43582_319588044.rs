rust
[00:50:15] error: variable does not need to be mutable
[00:50:15]    --> /checkout/src/libcore/../libcore/tests/slice.rs:108:9
[00:50:15]     |
[00:50:15] 108 |     let mut v: &mut [i32] = &mut [0, 1, 2, 3, 4, 5];
[00:50:15]     |         ^^^^^
[00:50:15]     |
[00:50:15] 
[00:50:15] error: variable does not need to be mutable
[00:50:15]    --> /checkout/src/libcore/../libcore/tests/slice.rs:112:9
[00:50:15]     |
[00:50:15] 112 |     let mut v2: &mut [i32] = &mut [0, 1, 2, 3, 4];
[00:50:15]     |         ^^^^^^
[00:50:15] 
[00:50:15] error: variable does not need to be mutable
[00:50:15]    --> /checkout/src/libcore/../libcore/tests/slice.rs:116:9
[00:50:15]     |
[00:50:15] 116 |     let mut v3: &mut [i32] = &mut [];
[00:50:15]     |         ^^^^^^
[00:50:15] 
[00:50:15] error: variable does not need to be mutable
[00:50:15]    --> /checkout/src/libcore/../libcore/tests/slice.rs:123:9
[00:50:15]     |
[00:50:15] 123 |     let mut v: &mut [i32] = &mut [0, 1, 2, 3, 4, 5];
[00:50:15]     |         ^^^^^
[00:50:15] 
[00:50:15] error: variable does not need to be mutable
[00:50:15]    --> /checkout/src/libcore/../libcore/tests/slice.rs:128:9
[00:50:15]     |
[00:50:15] 128 |     let mut v2: &mut [i32] = &mut [0, 1, 2, 3, 4];
[00:50:15]     |         ^^^^^^
[00:50:15] 
[00:50:15] error: variable does not need to be mutable
[00:50:15]    --> /checkout/src/libcore/../libcore/tests/slice.rs:197:9
[00:50:15]     |
[00:50:15] 197 |     let mut v: &mut [i32] = &mut [0, 1, 2, 3, 4, 5];
[00:50:15]     |         ^^^^^
[00:50:15] 
[00:50:20] error: variable does not need to be mutable
[00:50:20]    --> /checkout/src/libstd/error.rs:517:13
[00:50:20]     |
[00:50:20] 517 |         let mut a = &mut a as &mut (Error + 'static);
[00:50:20]     |             ^^^^^
[00:50:20]     |
[00:50:20] 
[00:50:20] error: variable does not need to be mutable
[00:50:20]    --> /checkout/src/libstd/io/cursor.rs:459:13
[00:50:20]     |
[00:50:20] 459 |         let mut reader = &mut &in_buf[..];
[00:50:20]     |             ^^^^^^^^^^
[00:50:20] 
