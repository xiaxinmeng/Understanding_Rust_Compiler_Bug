plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0271]: type mismatch resolving `<Result<bool, ()> as Try>::Residual == Result<Infallible, _>`
    |
    |
284 |     assert_eq!(xs.iter().try_find(testfn), Ok(None));
    |
    = note: expected type `()`
               found enum `Result<Infallible, _>`


error[E0271]: type mismatch resolving `<Result<bool, ()> as Try>::Residual == Result<Infallible, _>`
    |
    |
286 |     assert_eq!(xs.iter().try_find(testfn), Ok(Some(&2)));
    |
    = note: expected type `()`
               found enum `Result<Infallible, _>`


error[E0271]: type mismatch resolving `<Result<bool, ()> as Try>::Residual == Result<Infallible, _>`
    |
    |
288 |     assert_eq!(xs.iter().try_find(testfn), Err(()));
    |
    = note: expected type `()`
               found enum `Result<Infallible, _>`


error[E0271]: type mismatch resolving `<Result<bool, ()> as Try>::Residual == Result<Infallible, _>`
    |
    |
292 |     assert_eq!(iter.try_find(testfn), Ok(Some(&2)));
    |
    = note: expected type `()`
               found enum `Result<Infallible, _>`


error[E0271]: type mismatch resolving `<Result<bool, ()> as Try>::Residual == Result<Infallible, _>`
    |
    |
293 |     assert_eq!(iter.try_find(testfn), Err(()));
    |
    = note: expected type `()`
               found enum `Result<Infallible, _>`


error[E0271]: type mismatch resolving `<Result<bool, ParseIntError> as Try>::Residual == Result<Infallible, _>`
    |
    |
315 |     let val = a.iter().try_find(|&&s| is_my_num(s, 2))?;
    |                        ^^^^^^^^ expected struct `ParseIntError`, found enum `Result`
    = note: expected type `ParseIntError`
               found enum `Result<Infallible, _>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                   ^^^^^^^^^^ expected `i32`, found enum `Result`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    | 
    | 
   ::: library/core/tests/result.rs:398:5
    |
398 |       assert_eq!(Err::<i32, i32>(4).branch(), Break(Err(4)));
    |
    = note: expected enum `ControlFlow<i32, i32>`
    = note: expected enum `ControlFlow<i32, i32>`
               found enum `ControlFlow<Result<_, {integer}>, _>`
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                   ^^^^^^^^^^ expected struct `NonZeroU32`, found enum `Result`
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    | 
    | 
   ::: library/core/tests/result.rs:401:5
    |
401 |       assert_eq!(Err::<(), NonZeroU32>(one).branch(), Break(Err(one)));
    |
    |
    = note: expected enum `ControlFlow<NonZeroU32, ()>`
               found enum `ControlFlow<Result<_, NonZeroU32>, _>`
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
...   |
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    | |_- in this expansion of `assert_eq!`
    | 
   ::: library/core/tests/result.rs:403:5
    |
403 |       assert_eq!(Err::<NonZeroU32, ()>(()).branch(), Break(Err(())));
    |
    = note: expected enum `ControlFlow<(), NonZeroU32>`
    = note: expected enum `ControlFlow<(), NonZeroU32>`
               found enum `ControlFlow<Result<_, ()>, _>`
Some errors have detailed explanations: E0271, E0308.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `core` due to 9 previous errors
warning: build failed, waiting for other jobs to finish...
