plain
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.99
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
error: expected one of `!`, `(`, `,`, `?`, `for`, `{`, lifetime, or path, found `~`
    |
    |
538 |     U: ~const From<T>,
    |        ^ expected one of 8 possible tokens

error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, lifetime, or path, found `~`
     |
     |
1913 | impl<T, E, F: ~const From<E>> const ops::FromResidual<Result<convert::Infallible, E>> for Result<T, F> {
     |               ^ expected one of 9 possible tokens
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0432]: unresolved import `crate::convert::FloatToInt`
  --> library/core/src/num/f32.rs:14:5
  --> library/core/src/num/f32.rs:14:5
   |
14 | use crate::convert::FloatToInt;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `FloatToInt` in `convert`
error[E0432]: unresolved import `crate::convert::FloatToInt`
  --> library/core/src/num/f64.rs:14:5
   |
14 | use crate::convert::FloatToInt;
14 | use crate::convert::FloatToInt;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `FloatToInt` in `convert`
error[E0432]: unresolved import `crate::convert::Infallible`
 --> library/core/src/num/error.rs:3:5
  |
3 | use crate::convert::Infallible;
3 | use crate::convert::Infallible;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Infallible` in `convert`

error[E0432]: unresolved imports `crate::convert::AsMut`, `crate::convert::AsRef`, `crate::convert::From`, `crate::convert::Into`
  --> library/core/src/prelude/v1.rs:29:26
   |
29 | pub use crate::convert::{AsMut, AsRef, From, Into};
   |                          ^^^^^  ^^^^^  ^^^^  ^^^^ no `Into` in `convert`
   |                          |      |      |
   |                          |      |      no `From` in `convert`
   |                          |      no `AsRef` in `convert`
   |                          no `AsMut` in `convert`
error[E0432]: unresolved imports `crate::result::Result`, `crate::result::Result`
  --> library/core/src/prelude/v1.rs:44:24
   |
   |
44 | pub use crate::result::Result::{self, Err, Ok};
   |                        ^^^^^^   ^^^^ no `Result` in `result`
   |                        could not find `Result` in `result`


error[E0432]: unresolved imports `crate::convert::TryFrom`, `crate::convert::TryInto`
   |
   |
46 |     pub use crate::convert::{TryFrom, TryInto};
   |                              ^^^^^^^  ^^^^^^^ no `TryInto` in `convert`
   |                              |
   |                              no `TryFrom` in `convert`
error[E0432]: unresolved import `crate::convert::From`
 --> library/core/src/ptr/non_null.rs:2:5
  |
2 | use crate::convert::From;
2 | use crate::convert::From;
  |     ^^^^^^^^^^^^^^^^^^^^ no `From` in `convert`
error[E0432]: unresolved import `crate::convert::From`
 --> library/core/src/ptr/unique.rs:1:5
  |
1 | use crate::convert::From;
1 | use crate::convert::From;
  |     ^^^^^^^^^^^^^^^^^^^^ no `From` in `convert`

error[E0432]: unresolved imports `crate::convert::Infallible`, `crate::convert::TryFrom`
  |
  |
9 | use crate::convert::{Infallible, TryFrom};
  |                      ^^^^^^^^^^  ^^^^^^^ no `TryFrom` in `convert`
  |                      |
  |                      no `Infallible` in `convert`
error[E0432]: unresolved import `crate::convert::TryFrom`
 --> library/core/src/char/convert.rs:3:5
  |
3 | use crate::convert::TryFrom;
3 | use crate::convert::TryFrom;
  |     ^^^^^^^^^^^^^^^^^^^^^^^ no `TryFrom` in `convert`
error[E0432]: unresolved import `crate::convert::TryFrom`
 --> library/core/src/iter/range.rs:2:5
  |
2 | use crate::convert::TryFrom;
2 | use crate::convert::TryFrom;
  |     ^^^^^^^^^^^^^^^^^^^^^^^ no `TryFrom` in `convert`
error[E0432]: unresolved import `crate::result::Result`
  --> library/core/src/slice/mod.rs:17:5
   |
17 | use crate::result::Result;
17 | use crate::result::Result;
   |     ^^^^^^^^^^^^^^^^^^^^^ no `Result` in `result`

error[E0432]: unresolved import `crate::result::Result`
  --> library/core/src/slice/mod.rs:18:20
   |
18 | use crate::result::Result::{Err, Ok};
   |                    ^^^^^^ could not find `Result` in `result`
error[E0432]: unresolved import `crate::result::Result`
 --> library/core/src/task/poll.rs:5:5
  |
5 | use crate::result::Result;
5 | use crate::result::Result;
  |     ^^^^^^^^^^^^^^^^^^^^^ no `Result` in `result`

error[E0433]: failed to resolve: could not find `Result` in `result`
   --> library/core/src/num/bignum.rs:460:32
    |
96  | / macro_rules! define_bignum {
97  | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
98  | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
...   |
460 | |                 crate::result::Result::Ok(())
    | |                                ^^^^^^ not found in `crate::result`
...   |
...   |
463 | |     };
464 | | }
    | |_- in this expansion of `define_bignum!`
...
469 |   define_bignum!(Big32x40: type=Digit32, n=40);
    |
help: consider importing this type alias
    |
22  | use crate::fmt::Result;
22  | use crate::fmt::Result;
    |

error[E0433]: failed to resolve: could not find `Result` in `result`
   --> library/core/src/num/bignum.rs:460:32
    |
96  | / macro_rules! define_bignum {
97  | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
98  | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
...   |
460 | |                 crate::result::Result::Ok(())
    | |                                ^^^^^^ not found in `crate::result`
...   |
...   |
463 | |     };
464 | | }
    | |_- in this expansion of `define_bignum!`
...
474 |       define_bignum!(Big8x3: type=u8, n=3);
    |
    = note: consider importing this type alias:
            crate::fmt::Result


error[E0412]: cannot find type `Infallible` in module `convert`
   |
68 |     type Residual = ControlFlow<B, convert::Infallible>;
   |                                             ^^^^^^^^^^ not found in `convert`


error[E0412]: cannot find type `Infallible` in module `convert`
   |
   |
87 |     fn from_residual(residual: ControlFlow<B, convert::Infallible>) -> Self {
   |                                                        ^^^^^^^^^^ not found in `convert`
error[E0412]: cannot find type `Infallible` in module `crate::convert`
    --> library/core/src/iter/traits/iterator.rs:2439:50
     |
     |
2439 |         R: Try<Residual = Result<crate::convert::Infallible, E>>,
     |                                                  ^^^^^^^^^^ not found in `crate::convert`
error[E0412]: cannot find type `Infallible` in module `crate::convert`
    --> library/core/src/iter/traits/iterator.rs:2446:54
     |
     |
2446 |             R: Try<Residual = Result<crate::convert::Infallible, E>>,
     |                                                      ^^^^^^^^^^ not found in `crate::convert`

error[E0412]: cannot find type `Infallible` in module `convert`
     |
2016 |     type Residual = Option<convert::Infallible>;
     |                                     ^^^^^^^^^^ not found in `convert`


error[E0412]: cannot find type `Infallible` in module `convert`
     |
     |
2036 |     fn from_residual(residual: Option<convert::Infallible>) -> Self {
     |                                                ^^^^^^^^^^ not found in `convert`
error[E0412]: cannot find type `Result` in module `result`
  --> library/core/src/fmt/mod.rs:70:27
   |
70 | pub type Result = result::Result<(), Error>;
---

error[E0412]: cannot find type `Result` in module `result`
    --> library/core/src/fmt/mod.rs:1450:18
     |
1450 |     ) -> result::Result<PostPadding, Error> {
     |
help: consider importing this type alias
     |
5    | use crate::fmt::float::Result;
5    | use crate::fmt::float::Result;
     |

error[E0412]: cannot find type `Infallible` in module `convert`
    |
    |
227 |     type Residual = Result<convert::Infallible, E>;
    |                                     ^^^^^^^^^^ not found in `convert`

error[E0412]: cannot find type `Infallible` in module `convert`
    |
    |
245 | impl<T, E, F: From<E>> ops::FromResidual<Result<convert::Infallible, E>> for Poll<Result<T, F>> {
    |                                                          ^^^^^^^^^^ not found in `convert`

error[E0412]: cannot find type `Infallible` in module `convert`
    |
    |
247 |     fn from_residual(x: Result<convert::Infallible, E>) -> Self {
    |                                         ^^^^^^^^^^ not found in `convert`

error[E0412]: cannot find type `Infallible` in module `convert`
    |
    |
257 |     type Residual = Result<convert::Infallible, E>;
    |                                     ^^^^^^^^^^ not found in `convert`

error[E0412]: cannot find type `Infallible` in module `convert`
    |
    |
276 | impl<T, E, F: From<E>> ops::FromResidual<Result<convert::Infallible, E>>
    |                                                          ^^^^^^^^^^ not found in `convert`

error[E0412]: cannot find type `Infallible` in module `convert`
    |
    |
280 |     fn from_residual(x: Result<convert::Infallible, E>) -> Self {
    |                                         ^^^^^^^^^^ not found in `convert`
Some errors have detailed explanations: E0412, E0432, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `core` due to 32 previous errors
warning: build failed, waiting for other jobs to finish...
