plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0004]: non-exhaustive patterns: `Infinite`, `Zero` and `Normal` not covered
    |
    |
923 |             match ct.classify() {
    |                   ^^^^^^^^^^^^^ patterns `Infinite`, `Zero` and `Normal` not covered
    |
note: `FpCategory` defined here
    |
938 | pub enum FpCategory {
    |          ----------
...
---
    |     ^^^^ not covered
...
975 |     Normal,
    |     ^^^^^^ not covered
    = note: the matched value is of type `FpCategory`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
933 ~                 }
933 ~                 }
934 +                 Infinite | Zero | Normal => todo!()


error[E0004]: non-exhaustive patterns: `Infinite`, `Zero` and `Normal` not covered
     |
     |
1011 |             match f32::classify_bits(ct) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^ patterns `Infinite`, `Zero` and `Normal` not covered
     |
note: `FpCategory` defined here
     |
938  | pub enum FpCategory {
     |          ----------
...
---
     |     ^^^^ not covered
...
975  |     Normal,
     |     ^^^^^^ not covered
     = note: the matched value is of type `FpCategory`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
1021 ~                 }
1021 ~                 }
1022 +                 Infinite | Zero | Normal => todo!()


error[E0004]: non-exhaustive patterns: `Infinite`, `Zero` and `Normal` not covered
    |
    |
916 |             match ct.classify() {
    |                   ^^^^^^^^^^^^^ patterns `Infinite`, `Zero` and `Normal` not covered
    |
note: `FpCategory` defined here
    |
938 | pub enum FpCategory {
    |          ----------
...
---
    |     ^^^^ not covered
...
975 |     Normal,
    |     ^^^^^^ not covered
    = note: the matched value is of type `FpCategory`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
926 ~                 }
926 ~                 }
927 +                 Infinite | Zero | Normal => todo!()


error[E0004]: non-exhaustive patterns: `Infinite`, `Zero` and `Normal` not covered
     |
     |
1009 |             match f64::classify_bits(ct) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^ patterns `Infinite`, `Zero` and `Normal` not covered
     |
note: `FpCategory` defined here
     |
938  | pub enum FpCategory {
     |          ----------
...
---
     |     ^^^^ not covered
...
975  |     Normal,
     |     ^^^^^^ not covered
     = note: the matched value is of type `FpCategory`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
1019 ~                 }
1019 ~                 }
1020 +                 Infinite | Zero | Normal => todo!()


error[E0004]: non-exhaustive patterns: `Less` and `Equal` not covered
     |
     |
1216 |     match compare(&v1, &v2) {
     |           ^^^^^^^^^^^^^^^^^ patterns `Less` and `Equal` not covered
     |
note: `cmp::Ordering` defined here
     |
340  | pub enum Ordering {
     |          --------
...
...
343  |     Less = -1,
     |     ^^^^ not covered
...
346  |     Equal = 0,
     |     ^^^^^ not covered
     = note: the matched value is of type `cmp::Ordering`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
     |
1218 ~         Ordering::Greater => v2,
1219 ~         Less | Equal => todo!(),


error[E0004]: non-exhaustive patterns: `Less` and `Equal` not covered
     |
     |
1279 |     match compare(&v1, &v2) {
     |           ^^^^^^^^^^^^^^^^^ patterns `Less` and `Equal` not covered
     |
note: `cmp::Ordering` defined here
     |
340  | pub enum Ordering {
     |          --------
...
...
343  |     Less = -1,
     |     ^^^^ not covered
...
346  |     Equal = 0,
     |     ^^^^^ not covered
     = note: the matched value is of type `cmp::Ordering`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
     |
1281 ~         Ordering::Greater => v1,
1282 ~         Less | Equal => todo!(),


error[E0004]: non-exhaustive patterns: `RightBrace`, `Value`, `LeftBrace` and 2 more not covered
    |
217 |         match self.state {
217 |         match self.state {
    |               ^^^^^^^^^^ patterns `RightBrace`, `Value`, `LeftBrace` and 2 more not covered
note: `char::EscapeUnicodeState` defined here
   --> library/core/src/char/mod.rs:160:6
    |
    |
160 | enum EscapeUnicodeState {
    = note: the matched value is of type `char::EscapeUnicodeState`
    = note: the matched value is of type `char::EscapeUnicodeState`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
    |
224 ~             | EscapeUnicodeState::Backslash => Some('}'),
226 ~             _ => todo!(),
    |


error[E0004]: non-exhaustive patterns: `Char(_)` and `Backslash(_)` not covered
    |
334 |         match self.state {
334 |         match self.state {
    |               ^^^^^^^^^^ patterns `Char(_)` and `Backslash(_)` not covered
note: `char::EscapeDefaultState` defined here
   --> library/core/src/char/mod.rs:274:5
    |
    |
272 | enum EscapeDefaultState {
273 |     Done,
274 |     Char(char),
    |     ^^^^ not covered
    |     ^^^^ not covered
275 |     Backslash(char),
    = note: the matched value is of type `char::EscapeDefaultState`
    = note: the matched value is of type `char::EscapeDefaultState`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
    |
337 ~             EscapeDefaultState::Backslash(c) | EscapeDefaultState::Char(c) => Some(c),
338 ~             Char(_) | Backslash(_) => todo!(),


error[E0004]: non-exhaustive patterns: `Right` and `Unknown` not covered
     |
     |
1502 |         let (pre_pad, post_pad) = match align {
     |                                         ^^^^^ patterns `Right` and `Unknown` not covered
     |
note: `rt::v1::Alignment` defined here
    --> library/core/src/fmt/rt/v1.rs:29:5
25   | pub enum Alignment {
     |          ---------
...
29   |     Right,
29   |     Right,
     |     ^^^^^ not covered
...
33   |     Unknown,
     |     ^^^^^^^ not covered
     = note: the matched value is of type `rt::v1::Alignment`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
     |
1505 ~             rt::v1::Alignment::Center => (padding / 2, (padding + 1) / 2),
1506 ~             Right | Unknown => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `core` due to 9 previous errors
Build completed unsuccessfully in 0:03:44
