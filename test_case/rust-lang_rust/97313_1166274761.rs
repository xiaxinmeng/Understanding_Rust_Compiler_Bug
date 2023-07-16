plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e02d645110ae14f4a7f04d6bd5b05f2842488dda and d74a33001107939bef9a800e07a5a6c8b0fdb2d7
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling url v1.7.2
error[E0106]: missing lifetime specifier
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:220:51
    |
220 |     custom_encoding: Option<SilentDebug<Box<FnMut(&str) -> Cow<[u8]>>>>,
    |                                                   ^ expected named lifetime parameter
help: consider introducing a named lifetime parameter
    |
    |
216 ~ pub struct Serializer<'a, T: Target> {
218 |     start_position: usize,
219 |     encoding: EncodingOverride,
219 |     encoding: EncodingOverride,
220 ~     custom_encoding: Option<SilentDebug<Box<FnMut(&'a str) -> Cow<[u8]>>>>,

error[E0106]: missing lifetime specifier
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/url-1.7.2/src/form_urlencoded.rs:220:63
    |
    |
220 |     custom_encoding: Option<SilentDebug<Box<FnMut(&str) -> Cow<[u8]>>>>,
    |                                                               ^ expected named lifetime parameter
help: consider introducing a named lifetime parameter
    |
    |
216 ~ pub struct Serializer<'a, T: Target> {
218 |     start_position: usize,
219 |     encoding: EncodingOverride,
219 |     encoding: EncodingOverride,
220 ~     custom_encoding: Option<SilentDebug<Box<FnMut(&str) -> Cow<'a, [u8]>>>>,

For more information about this error, try `rustc --explain E0106`.
error: could not compile `url` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
---
   Compiling dirs-sys v0.3.7
   Compiling rand v0.4.6
   Compiling fs2 v0.4.3
   Compiling dirs v4.0.0
error[E0597]: `buf` does not live long enough
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/os.rs:63:14
   |
61 | fn next_u32(fill_buf: &mut FnMut(&mut [u8])) -> u32 {
   |             -------- has type `&mut dyn FnMut(&'1 mut [u8])`
62 |     let mut buf: [u8; 4] = [0; 4];
63 |     fill_buf(&mut buf);
   |     |        |
   |     |        borrowed value does not live long enough
   |     |        borrowed value does not live long enough
   |     argument requires that `buf` is borrowed for `'1`
64 |     unsafe { mem::transmute::<[u8; 4], u32>(buf) }
65 | }
   | - `buf` dropped here while still borrowed

error[E0503]: cannot use `buf` because it was mutably borrowed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/os.rs:64:45
   |
61 | fn next_u32(fill_buf: &mut FnMut(&mut [u8])) -> u32 {
   |             -------- has type `&mut dyn FnMut(&'1 mut [u8])`
62 |     let mut buf: [u8; 4] = [0; 4];
63 |     fill_buf(&mut buf);
   |     |        |
   |     |        |
   |     |        borrow of `buf` occurs here
   |     argument requires that `buf` is borrowed for `'1`
64 |     unsafe { mem::transmute::<[u8; 4], u32>(buf) }
   |                                             ^^^ use of borrowed `buf`

error[E0597]: `buf` does not live long enough
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/os.rs:70:14
   |
68 | fn next_u64(fill_buf: &mut FnMut(&mut [u8])) -> u64 {
   |             -------- has type `&mut dyn FnMut(&'1 mut [u8])`
69 |     let mut buf: [u8; 8] = [0; 8];
70 |     fill_buf(&mut buf);
   |     |        |
   |     |        borrowed value does not live long enough
   |     |        borrowed value does not live long enough
   |     argument requires that `buf` is borrowed for `'1`
71 |     unsafe { mem::transmute::<[u8; 8], u64>(buf) }
72 | }
   | - `buf` dropped here while still borrowed

error[E0503]: cannot use `buf` because it was mutably borrowed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.4.6/src/os.rs:71:45
   |
68 | fn next_u64(fill_buf: &mut FnMut(&mut [u8])) -> u64 {
   |             -------- has type `&mut dyn FnMut(&'1 mut [u8])`
69 |     let mut buf: [u8; 8] = [0; 8];
70 |     fill_buf(&mut buf);
   |     |        |
   |     |        |
   |     |        borrow of `buf` occurs here
   |     argument requires that `buf` is borrowed for `'1`
71 |     unsafe { mem::transmute::<[u8; 8], u64>(buf) }
   |                                             ^^^ use of borrowed `buf`
Some errors have detailed explanations: E0503, E0597.
For more information about an error, try `rustc --explain E0503`.
error: could not compile `rand` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: failed to compile `xargo v0.3.26`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools`
1 command(s) did not execute successfully:

  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-Zcheck-cfg=names,values,features" "-Zbinary-dep-depinfo" "-j" "16" "--locked" "--color" "always" "xargo"

---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to build-fail during beta week.
