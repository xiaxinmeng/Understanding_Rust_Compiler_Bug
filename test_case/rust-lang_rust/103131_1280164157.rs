plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b8b5caee04116c7383eb1c6470fcf15c437a60d4 and 803ec9474eb34a90c3818acd40876e9791ec4ceb
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.131
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
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0423]: expected value, found module `slice`
    --> library/core/src/mem/maybe_uninit.rs:1303:21
    --> library/core/src/mem/maybe_uninit.rs:1303:21
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
warning: build failed, waiting for other jobs to finish...
