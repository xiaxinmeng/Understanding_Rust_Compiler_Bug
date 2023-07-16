plain
    --> /checkout/library/alloc/src/macros.rs:49:36
     |
41   | / macro_rules! vec {
42   | |     () => (
43   | |         $crate::__rust_force_expr!($crate::vec::Vec::new())
...    |
...    |
49   | |         $crate::__rust_force_expr!(<[_]>::into_vec(box [$($x),+]))
     | |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Components`, found struct `alloc_crate::vec::Vec`
51   | | }
     | |_- in this expansion of `vec!`
     |
    ::: library/std/src/path.rs:1241:37
    ::: library/std/src/path.rs:1241:37
     |
1241 |   ...                   comps = vec![c];
     |
     = note: expected struct `Components<'_>`
     = note: expected struct `Components<'_>`
                found struct `alloc_crate::vec::Vec<Component<'_>>`

error[E0599]: no method named `push` found for struct `Components` in the current scope
     |
     |
583  | pub struct Components<'a> {
     | ------------------------- method `push` not found for this
...
1246 |                                 comps.push(c);
     |                                       ^^^^ method not found in `Components<'_>`
error[E0308]: mismatched types
    --> /checkout/library/alloc/src/macros.rs:49:36
     |
41   | / macro_rules! vec {
41   | / macro_rules! vec {
42   | |     () => (
43   | |         $crate::__rust_force_expr!($crate::vec::Vec::new())
...    |
...    |
49   | |         $crate::__rust_force_expr!(<[_]>::into_vec(box [$($x),+]))
     | |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Components`, found struct `alloc_crate::vec::Vec`
51   | | }
     | |_- in this expansion of `vec!`
     |
    ::: library/std/src/path.rs:1248:41
    ::: library/std/src/path.rs:1248:41
     |
1248 |   ...                   comps = vec![c];
     |
     = note: expected struct `Components<'_>`
     = note: expected struct `Components<'_>`
                found struct `alloc_crate::vec::Vec<Component<'_>>`

error[E0599]: no method named `push` found for struct `Components` in the current scope
     |
     |
583  | pub struct Components<'a> {
     | ------------------------- method `push` not found for this
1255 |                                 comps.push(c)
     |                                       ^^^^ method not found in `Components<'_>`


error[E0599]: no method named `pop` found for struct `Components` in the current scope
     |
     |
583  | pub struct Components<'a> {
     | ------------------------- method `pop` not found for this
...
1258 |                                 let _ = comps.pop();
     |                                               ^^^ method not found in `Components<'_>`

error[E0599]: no method named `push` found for struct `Components` in the current scope
     |
     |
583  | pub struct Components<'a> {
     | ------------------------- method `push` not found for this
...
1262 |                             comps.push(c);
     |                                   ^^^^ method not found in `Components<'_>`
error[E0599]: no method named `to_path_buf` found for struct `Components` in the current scope
    --> library/std/src/path.rs:1266:30
     |
     |
583  | pub struct Components<'a> {
     | ------------------------- method `to_path_buf` not found for this
...
1266 |                 return comps.to_path_buf();
     |                              ^^^^^^^^^^^ method not found in `Components<'_>`
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 7 previous errors
Build completed unsuccessfully in 0:01:13
