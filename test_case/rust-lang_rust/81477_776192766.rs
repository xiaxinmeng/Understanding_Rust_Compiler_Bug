plain

###############################                                           44.1%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-01-28/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating git repository `https://github.com/webdesus/fs_extra`
---
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
error: Trailing semicolon in macro
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/snap-1.0.1/src/read.rs:103:50
    |
101 | /         macro_rules! fail {
102 | |             ($err:expr) => {
103 | |                 return Err(io::Error::from($err));
104 | |             };
105 | |         }
105 | |         }
    | |_________- in this expansion of `fail!`
...
156 | /                         fail!(Error::UnsupportedChunkLength {
157 | |                             len: len64,
158 | |                             header: true,
    | |__________________________- in this macro invocation

   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling ena v0.14.0
