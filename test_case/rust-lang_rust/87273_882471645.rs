plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0308]: mismatched types
    --> /checkout/library/core/src/macros/mod.rs:53:35
     |
36   | / macro_rules! assert_eq {
37   | |     ($left:expr, $right:expr $(,)?) => ({
38   | |         match (&$left, &$right) {
39   | |             (left_val, right_val) => {
...    |
53   | |                 if !(*left_val == *right_val) {
     | |                                   ^^^^^^^^^^ expected enum `Constness`, found `&Constness`
62   | |     });
63   | | }
     | |_- in this expansion of `assert_eq!`
     | 
     | 
    ::: compiler/rustc_hir/src/hir.rs:3085:17
     |
3085 | /                 assert_eq!(
3086 | |                     Constness::NotConst,
3087 | |                     constness,
3088 | |                     "trait and impl fns cannot be marked const"
     | |__________________- in this macro invocation

error: aborting due to previous error

