plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `panic` is not yet stable as a const fn
    |
23  | / pub macro panic_2015 {
24  | |     () => (
25  | |         $crate::panicking::panic("explicit panic")
---
    | |_- in this expansion of `$crate::panic::panic_2015!` (#2)
    |
   ::: library/core/src/time.rs:189:21
    |
189 |               None => panic!("overflow in Duration::new"),
    |                       |
    |                       in this macro invocation (#1)
    |                       in this macro invocation (#2)
    |
---
    | |_- in this expansion of `panic!` (#1)
    |
    = help: const-stable functions can only call other const-stable functions

error: `panic` is not yet stable as a const fn
     |
214  | / macro_rules! debug_assert {
214  | / macro_rules! debug_assert {
215  | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })
     | |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation (#2)
     | |_- in this expansion of `debug_assert!` (#1)
...
1326 | /     macro_rules! assert {
1326 | /     macro_rules! assert {
1327 | |         ($cond:expr $(,)?) => {{ /* compiler built-in */ }};
1328 | |         ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `$crate::assert!` (#2)
     |
    ::: library/core/src/time.rs:497:13
     |
     |
497  |               debug_assert!(nanos < NANOS_PER_SEC);
     |
     = help: const-stable functions can only call other const-stable functions


error: `panic` is not yet stable as a const fn
     |
214  | / macro_rules! debug_assert {
214  | / macro_rules! debug_assert {
215  | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })
     | |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation (#2)
     | |_- in this expansion of `debug_assert!` (#1)
...
1326 | /     macro_rules! assert {
1326 | /     macro_rules! assert {
1327 | |         ($cond:expr $(,)?) => {{ /* compiler built-in */ }};
1328 | |         ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `$crate::assert!` (#2)
     |
    ::: library/core/src/time.rs:557:13
     |
     |
557  |               debug_assert!(nanos < NANOS_PER_SEC);
     |
     = help: const-stable functions can only call other const-stable functions


error: `panic` is not yet stable as a const fn
     |
214  | / macro_rules! debug_assert {
214  | / macro_rules! debug_assert {
215  | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })
     | |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation (#2)
     | |_- in this expansion of `debug_assert!` (#1)
...
1326 | /     macro_rules! assert {
1326 | /     macro_rules! assert {
1327 | |         ($cond:expr $(,)?) => {{ /* compiler built-in */ }};
1328 | |         ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `$crate::assert!` (#2)
     |
    ::: library/core/src/time.rs:613:17
     |
     |
613  |                   debug_assert!(nanos < NANOS_PER_SEC);
     |
     = help: const-stable functions can only call other const-stable functions


error: `panic` is not yet stable as a const fn
     |
214  | / macro_rules! debug_assert {
214  | / macro_rules! debug_assert {
215  | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })
     | |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation (#2)
     | |_- in this expansion of `debug_assert!` (#1)
...
1326 | /     macro_rules! assert {
1326 | /     macro_rules! assert {
1327 | |         ($cond:expr $(,)?) => {{ /* compiler built-in */ }};
1328 | |         ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `$crate::assert!` (#2)
     |
    ::: library/core/src/time.rs:670:13
     |
     |
670  |               debug_assert!(nanos < NANOS_PER_SEC);
     |
     = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to 5 previous errors
