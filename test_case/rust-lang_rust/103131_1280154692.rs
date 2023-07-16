plain
   Compiling libc v0.2.131
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
error: invalid variable declaration
   --> library/core/src/num/flt2dec/strategy/dragon.rs:249:44
    |
249 |         if let Some(c) = round_up(unsafe { mut buf[..i].assume_init_mut() }) {
    |                                            ^^^ help: missing keyword: `let mut`

error: expected one of `:`, `;`, `=`, `@`, or `|`, found `[`
   --> library/core/src/num/flt2dec/strategy/dragon.rs:249:51
    |
249 |         if let Some(c) = round_up(unsafe { mut buf[..i].assume_init_mut() }) {
    |                                                   ^ expected one of `:`, `;`, `=`, `@`, or `|`
error: invalid variable declaration
error: invalid variable declaration
   --> library/core/src/num/flt2dec/strategy/dragon.rs:374:44
    |
374 |         if let Some(c) = round_up(unsafe { mut buf[..len].assume_init_mut() }) {
    |                                            ^^^ help: missing keyword: `let mut`

error: expected one of `:`, `;`, `=`, `@`, or `|`, found `[`
   --> library/core/src/num/flt2dec/strategy/dragon.rs:374:51
    |
374 |         if let Some(c) = round_up(unsafe { mut buf[..len].assume_init_mut() }) {
    |                                                   ^ expected one of `:`, `;`, `=`, `@`, or `|`
error: invalid variable declaration
error: invalid variable declaration
   --> library/core/src/num/flt2dec/strategy/grisu.rs:278:26
    |
278 |                 unsafe { mut buf[..i].assume_init_mut() },
    |                          ^^^ help: missing keyword: `let mut`

error: expected one of `:`, `;`, `=`, `@`, or `|`, found `[`
   --> library/core/src/num/flt2dec/strategy/grisu.rs:278:33
    |
278 |                 unsafe { mut buf[..i].assume_init_mut() },
    |                                 ^ expected one of `:`, `;`, `=`, `@`, or `|`
error: invalid variable declaration
error: invalid variable declaration
   --> library/core/src/num/flt2dec/strategy/grisu.rs:327:26
    |
327 |                 unsafe { mut buf[..i].assume_init_mut() },
    |                          ^^^ help: missing keyword: `let mut`

error: expected one of `:`, `;`, `=`, `@`, or `|`, found `[`
   --> library/core/src/num/flt2dec/strategy/grisu.rs:327:33
    |
327 |                 unsafe { mut buf[..i].assume_init_mut() },
    |                                 ^ expected one of `:`, `;`, `=`, `@`, or `|`
error: invalid variable declaration
error: invalid variable declaration
   --> library/core/src/num/flt2dec/strategy/grisu.rs:723:35
    |
723 |                 round_up(unsafe { mut buf[..len].assume_init_mut() })
    |                                   ^^^ help: missing keyword: `let mut`

error: expected one of `:`, `;`, `=`, `@`, or `|`, found `[`
   --> library/core/src/num/flt2dec/strategy/grisu.rs:723:42
    |
723 |                 round_up(unsafe { mut buf[..len].assume_init_mut() })
    |                                          ^ expected one of `:`, `;`, `=`, `@`, or `|`
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0423]: expected value, found module `slice`
    --> library/core/src/mem/maybe_uninit.rs:1303:21
     |
     |
1303 |         unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }

error[E0423]: expected value, found module `slice`
    --> library/core/src/mem/maybe_uninit.rs:1323:25
     |
     |
1323 |         unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }

error[E0599]: no function or associated item named `slice_assume_init_mut` found for union `maybe_uninit::MaybeUninit` in the current scope
   --> library/core/src/array/mod.rs:875:56
    |
    |
875 |                 crate::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(
    |                                                        |
    |                                                        function or associated item not found in `maybe_uninit::MaybeUninit<_>`
    |                                                        help: there is a method with a similar name: `assume_init`
    |
    |
   ::: library/core/src/mem/maybe_uninit.rs:250:1
    |
250 | pub union MaybeUninit<T> {
    | ------------------------ function or associated item `slice_assume_init_mut` not found for this union
Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `core` due to 13 previous errors
Build completed unsuccessfully in 0:01:38
