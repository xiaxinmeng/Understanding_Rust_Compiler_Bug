plain
Successfully built 7a97eca47e99
Successfully tagged rust-ci:latest
Built container sha256:7a97eca47e9905805142b5eff36151ecd31e4c9375040173447e068278ecdab9
Uploading finished image to https://ci-caches.rust-lang.org/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7
upload failed: - to s3://rust-lang-ci-sccache2/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.11.0
   Compiling addr2line v0.16.0
error: 1 positional argument in format string, but no arguments were given
     |
     |
1090 |                 format!("command {:?} produced non-UTF-8 output"),

error[E0308]: mismatched types
    --> /checkout/library/alloc/src/macros.rs:112:9
     |
     |
109  | / macro_rules! format {
110  | |     ($($arg:tt)*) => {{
111  | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
112  | |         res
     | |         ^^^ expected `&&'static str`, found struct `alloc_crate::string::String`
114  | | }
     | |_- in this expansion of `format!`
     |
    ::: library/std/src/process.rs:1090:17
    ::: library/std/src/process.rs:1090:17
     |
1090 |                   format!("command {:?} produced non-UTF-8 output"),

error[E0308]: mismatched types
    --> /checkout/library/alloc/src/macros.rs:112:9
     |
     |
109  | / macro_rules! format {
110  | |     ($($arg:tt)*) => {{
111  | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
112  | |         res
     | |         ^^^ expected `&&'static str`, found struct `alloc_crate::string::String`
114  | | }
     | |_- in this expansion of `format!`
     |
    ::: library/std/src/process.rs:1103:25
    ::: library/std/src/process.rs:1103:25
     |
1103 |                           format!("command {:?} exited with non zero status ({})", self, code)

error[E0308]: mismatched types
    --> /checkout/library/alloc/src/macros.rs:112:9
     |
     |
109  | / macro_rules! format {
110  | |     ($($arg:tt)*) => {{
111  | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
112  | |         res
     | |         ^^^ expected `&&'static str`, found struct `alloc_crate::string::String`
114  | | }
     | |_- in this expansion of `format!`
     |
    ::: library/std/src/process.rs:1105:29
    ::: library/std/src/process.rs:1105:29
     |
1105 |                       None => format!("command {:?} was terminated", self),

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 4 previous errors
Build completed unsuccessfully in 0:00:20
