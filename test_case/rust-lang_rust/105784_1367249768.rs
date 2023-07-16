plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: use of deprecated macro `deprecated_feature`: the `avx512gfni` feature has been renamed to `gfni`
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:42:29
    |
31  | /         macro_rules! $macro_name {
33  | |                 ($feature_lit) => {
33  | |                 ($feature_lit) => {
34  | |                     $crate::detect_feature!($feature, $feature_lit $(: $($target_feature_lit),*)?)
42  | |                             deprecated_feature! {};
    | |                             ^^^^^^^^^^^^^^^^^^
...   |
68  | |             };
68  | |             };
69  | |         }
    | |_________- in this expansion of `is_x86_feature_detected!`
   ::: library/std/tests/run-time-detect.rs:123:34
    |
    |
123 |       println!("avx512gfni: {:?}", is_x86_feature_detected!("avx512gfni"));
    |
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated macro `deprecated_feature`: the `avx512vaes` feature has been renamed to `vaes`
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:42:29
    |
31  | /         macro_rules! $macro_name {
33  | |                 ($feature_lit) => {
33  | |                 ($feature_lit) => {
34  | |                     $crate::detect_feature!($feature, $feature_lit $(: $($target_feature_lit),*)?)
42  | |                             deprecated_feature! {};
    | |                             ^^^^^^^^^^^^^^^^^^
...   |
68  | |             };
68  | |             };
69  | |         }
    | |_________- in this expansion of `is_x86_feature_detected!`
   ::: library/std/tests/run-time-detect.rs:126:34
    |
    |
126 |       println!("avx512vaes: {:?}", is_x86_feature_detected!("avx512vaes"));


error: use of deprecated macro `deprecated_feature`: the `avx512vpclmulqdq` feature has been renamed to `vpclmulqdq`
   --> /checkout/library/stdarch/crates/std_detect/src/detect/macros.rs:42:29
    |
31  | /         macro_rules! $macro_name {
33  | |                 ($feature_lit) => {
33  | |                 ($feature_lit) => {
34  | |                     $crate::detect_feature!($feature, $feature_lit $(: $($target_feature_lit),*)?)
42  | |                             deprecated_feature! {};
    | |                             ^^^^^^^^^^^^^^^^^^
...   |
68  | |             };
68  | |             };
69  | |         }
    | |_________- in this expansion of `is_x86_feature_detected!`
   ::: library/std/tests/run-time-detect.rs:132:40
    |
    |
132 |       println!("avx512vpclmulqdq: {:?}", is_x86_feature_detected!("avx512vpclmulqdq"));

error: could not compile `std` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:41
