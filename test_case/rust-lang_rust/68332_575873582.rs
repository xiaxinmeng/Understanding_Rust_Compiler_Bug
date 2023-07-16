plain
2020-01-18T07:01:21.9883624Z ========================== Starting Command Output ===========================
2020-01-18T07:01:21.9885525Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6fad163c-b61a-4260-b19a-fa50528f5888.sh
2020-01-18T07:01:21.9885936Z 
2020-01-18T07:01:21.9888373Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T07:01:21.9893870Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:01:21.9895155Z Task         : Get sources
2020-01-18T07:01:21.9895225Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:01:21.9895250Z Version      : 1.0.0
2020-01-18T07:01:21.9895274Z Author       : Microsoft
---
2020-01-18T07:01:22.9716600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T07:01:22.9726766Z ##[command]git config gc.auto 0
2020-01-18T07:01:22.9729269Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T07:01:22.9730809Z ##[command]git config --get-all http.proxy
2020-01-18T07:01:22.9736748Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T07:05:28.1468607Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T07:05:29.6534504Z error: expected unsuffixed literal or identifier, found `fn_not_impl`
2020-01-18T07:05:29.6534927Z    --> src/libcore/ops/function.rs:358:41
2020-01-18T07:05:29.6535196Z     |
2020-01-18T07:05:29.6535507Z 358 | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:29.6535875Z 
2020-01-18T07:05:29.6554550Z error: expected unsuffixed literal or identifier, found `fn_neg_impl`
2020-01-18T07:05:29.6554892Z    --> src/libcore/ops/function.rs:359:41
2020-01-18T07:05:29.6555116Z     |
2020-01-18T07:05:29.6555116Z     |
2020-01-18T07:05:29.6555426Z 359 | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:05:29.6555801Z 
2020-01-18T07:05:29.6555801Z 
2020-01-18T07:05:32.6410127Z error[E0119]: conflicting implementations of trait `ops::function::Fn<_>` for type `&ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T07:05:32.6411239Z    --> src/libcore/ops/function.rs:304:13
2020-01-18T07:05:32.6411772Z     |
2020-01-18T07:05:32.6412455Z 237 | /     impl<A, F: ?Sized> Fn<A> for &F
2020-01-18T07:05:32.6414404Z 239 | |         F: Fn<A>,
2020-01-18T07:05:32.6415602Z 240 | |     {
2020-01-18T07:05:32.6416252Z ...   |
2020-01-18T07:05:32.6417365Z 243 | |         }
2020-01-18T07:05:32.6417365Z 243 | |         }
2020-01-18T07:05:32.6417794Z 244 | |     }
2020-01-18T07:05:32.6418193Z     | |_____- first implementation here
2020-01-18T07:05:32.6418536Z ...
2020-01-18T07:05:32.6418942Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.6419286Z     |  _-
2020-01-18T07:05:32.6419641Z     | |_|
2020-01-18T07:05:32.6419977Z     | |
2020-01-18T07:05:32.6420547Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.6420992Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.6421351Z     |  _________-
2020-01-18T07:05:32.6421774Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.6422501Z ...
2020-01-18T07:05:32.6422501Z ...
2020-01-18T07:05:32.6423467Z 304 | /             impl<A, F: ?Sized> Fn<A> for &$z<F>
2020-01-18T07:05:32.6424605Z 306 |                   F: Fn<A>,
2020-01-18T07:05:32.6425044Z 307 |               {
2020-01-18T07:05:32.6425430Z ...
2020-01-18T07:05:32.6425877Z 310 |                   }
2020-01-18T07:05:32.6425877Z 310 |                   }
2020-01-18T07:05:32.6426337Z 311 | |             }
2020-01-18T07:05:32.6427155Z     | |_____________^ conflicting implementation for `&ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.6427830Z 356 | |     };
2020-01-18T07:05:32.6428189Z 357 | | }
2020-01-18T07:05:32.6428554Z     | | -
2020-01-18T07:05:32.6428901Z     | |_|
2020-01-18T07:05:32.6428901Z     | |_|
2020-01-18T07:05:32.6429378Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6429881Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6430308Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.6430793Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.6431286Z     |
2020-01-18T07:05:32.6431776Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.6431951Z 
2020-01-18T07:05:32.6567834Z error[E0119]: conflicting implementations of trait `ops::function::Fn<_>` for type `&ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T07:05:32.6568499Z    --> src/libcore/ops/function.rs:304:13
2020-01-18T07:05:32.6568850Z     |
2020-01-18T07:05:32.6569257Z 237 | /     impl<A, F: ?Sized> Fn<A> for &F
2020-01-18T07:05:32.6570233Z 239 | |         F: Fn<A>,
2020-01-18T07:05:32.6570652Z 240 | |     {
2020-01-18T07:05:32.6571005Z ...   |
2020-01-18T07:05:32.6571385Z 243 | |         }
2020-01-18T07:05:32.6571385Z 243 | |         }
2020-01-18T07:05:32.6571799Z 244 | |     }
2020-01-18T07:05:32.6572196Z     | |_____- first implementation here
2020-01-18T07:05:32.6572530Z ...
2020-01-18T07:05:32.6573362Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.6573858Z     |  _-
2020-01-18T07:05:32.6574391Z     | |_|
2020-01-18T07:05:32.6574823Z     | |
2020-01-18T07:05:32.6575356Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.6575877Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.6576349Z     |  _________-
2020-01-18T07:05:32.6577158Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.6577938Z ...
2020-01-18T07:05:32.6577938Z ...
2020-01-18T07:05:32.6578642Z 304 | /             impl<A, F: ?Sized> Fn<A> for &$z<F>
2020-01-18T07:05:32.6581078Z 306 |                   F: Fn<A>,
2020-01-18T07:05:32.6581349Z 307 |               {
2020-01-18T07:05:32.6581530Z ...
2020-01-18T07:05:32.6581916Z 310 |                   }
2020-01-18T07:05:32.6581916Z 310 |                   }
2020-01-18T07:05:32.6582151Z 311 | |             }
2020-01-18T07:05:32.6582456Z     | |_____________^ conflicting implementation for `&ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.6582970Z 356 | |     };
2020-01-18T07:05:32.6583786Z 357 | | }
2020-01-18T07:05:32.6584081Z     | | -
2020-01-18T07:05:32.6584341Z     | |_|
2020-01-18T07:05:32.6584341Z     | |_|
2020-01-18T07:05:32.6584649Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6584944Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6585289Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.6585647Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:05:32.6586081Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.6586317Z     |
2020-01-18T07:05:32.6586817Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.6587038Z 
2020-01-18T07:05:32.6728848Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T07:05:32.6729118Z    --> src/libcore/ops/function.rs:314:13
2020-01-18T07:05:32.6729312Z     |
2020-01-18T07:05:32.6729554Z 247 | /     impl<A, F: ?Sized> FnMut<A> for &F
2020-01-18T07:05:32.6730035Z 249 | |         F: Fn<A>,
2020-01-18T07:05:32.6730256Z 250 | |     {
2020-01-18T07:05:32.6730457Z ...   |
2020-01-18T07:05:32.6730676Z 253 | |         }
2020-01-18T07:05:32.6730676Z 253 | |         }
2020-01-18T07:05:32.6730894Z 254 | |     }
2020-01-18T07:05:32.6731159Z     | |_____- first implementation here
2020-01-18T07:05:32.6731314Z ...
2020-01-18T07:05:32.6731532Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.6731867Z     |  _-
2020-01-18T07:05:32.6732096Z     | |_|
2020-01-18T07:05:32.6732277Z     | |
2020-01-18T07:05:32.6732564Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.6732813Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.6733566Z     |  _________-
2020-01-18T07:05:32.6733927Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.6734501Z ...
2020-01-18T07:05:32.6734501Z ...
2020-01-18T07:05:32.6734828Z 314 | /             impl<A, F: ?Sized> FnMut<A> for &$z<F>
2020-01-18T07:05:32.6735396Z 316 |                   F: Fn<A>,
2020-01-18T07:05:32.6735663Z 317 |               {
2020-01-18T07:05:32.6735873Z ...
2020-01-18T07:05:32.6736152Z 320 |                   }
2020-01-18T07:05:32.6736152Z 320 |                   }
2020-01-18T07:05:32.6736443Z 321 | |             }
2020-01-18T07:05:32.6737055Z     | |_____________^ conflicting implementation for `&ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.6737451Z 356 | |     };
2020-01-18T07:05:32.6737663Z 357 | | }
2020-01-18T07:05:32.6737887Z     | | -
2020-01-18T07:05:32.6738069Z     | |_|
2020-01-18T07:05:32.6738069Z     | |_|
2020-01-18T07:05:32.6738310Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6738553Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6738822Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.6739147Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.6739322Z     |
2020-01-18T07:05:32.6739585Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.6739641Z 
2020-01-18T07:05:32.6745332Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&mut ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T07:05:32.6745928Z    --> src/libcore/ops/function.rs:336:13
2020-01-18T07:05:32.6746339Z     |
2020-01-18T07:05:32.6747230Z 269 | /     impl<A, F: ?Sized> FnMut<A> for &mut F
2020-01-18T07:05:32.6748290Z 271 | |         F: FnMut<A>,
2020-01-18T07:05:32.6748696Z 272 | |     {
2020-01-18T07:05:32.6749039Z ...   |
2020-01-18T07:05:32.6749582Z 275 | |         }
2020-01-18T07:05:32.6749582Z 275 | |         }
2020-01-18T07:05:32.6749967Z 276 | |     }
2020-01-18T07:05:32.6750372Z     | |_____- first implementation here
2020-01-18T07:05:32.6750711Z ...
2020-01-18T07:05:32.6751088Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.6751459Z     |  _-
2020-01-18T07:05:32.6751801Z     | |_|
2020-01-18T07:05:32.6752291Z     | |
2020-01-18T07:05:32.6752875Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.6753782Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.6754266Z     |  _________-
2020-01-18T07:05:32.6754781Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.6755699Z ...
2020-01-18T07:05:32.6755699Z ...
2020-01-18T07:05:32.6756201Z 336 | /             impl<A, F: ?Sized> FnMut<A> for &mut $z<F>
2020-01-18T07:05:32.6757695Z 338 |                   F: FnMut<A>,
2020-01-18T07:05:32.6758219Z 339 |               {
2020-01-18T07:05:32.6758517Z ...
2020-01-18T07:05:32.6758869Z 342 |                   }
2020-01-18T07:05:32.6758869Z 342 |                   }
2020-01-18T07:05:32.6759260Z 343 | |             }
2020-01-18T07:05:32.6759685Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.6760537Z 356 | |     };
2020-01-18T07:05:32.6761267Z 357 | | }
2020-01-18T07:05:32.6761682Z     | | -
2020-01-18T07:05:32.6762031Z     | |_|
2020-01-18T07:05:32.6762031Z     | |_|
2020-01-18T07:05:32.6762554Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6763437Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6764010Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.6764610Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.6765213Z     |
2020-01-18T07:05:32.6765847Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.6766033Z 
2020-01-18T07:05:32.6887808Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T07:05:32.6888479Z    --> src/libcore/ops/function.rs:314:13
2020-01-18T07:05:32.6888823Z     |
2020-01-18T07:05:32.6890008Z 247 | /     impl<A, F: ?Sized> FnMut<A> for &F
2020-01-18T07:05:32.6891132Z 249 | |         F: Fn<A>,
2020-01-18T07:05:32.6891515Z 250 | |     {
2020-01-18T07:05:32.6891853Z ...   |
2020-01-18T07:05:32.6892252Z 253 | |         }
2020-01-18T07:05:32.6892252Z 253 | |         }
2020-01-18T07:05:32.6892625Z 254 | |     }
2020-01-18T07:05:32.6893520Z     | |_____- first implementation here
2020-01-18T07:05:32.6893995Z ...
2020-01-18T07:05:32.6894446Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.6894895Z     |  _-
2020-01-18T07:05:32.6895319Z     | |_|
2020-01-18T07:05:32.6895749Z     | |
2020-01-18T07:05:32.6896299Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.6896866Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.6897234Z     |  _________-
2020-01-18T07:05:32.6897617Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.6898362Z ...
2020-01-18T07:05:32.6898362Z ...
2020-01-18T07:05:32.6899339Z 314 | /             impl<A, F: ?Sized> FnMut<A> for &$z<F>
2020-01-18T07:05:32.6899934Z 316 |                   F: Fn<A>,
2020-01-18T07:05:32.6900205Z 317 |               {
2020-01-18T07:05:32.6900364Z ...
2020-01-18T07:05:32.6900568Z 320 |                   }
2020-01-18T07:05:32.6900568Z 320 |                   }
2020-01-18T07:05:32.6900808Z 321 | |             }
2020-01-18T07:05:32.6901085Z     | |_____________^ conflicting implementation for `&ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.6901567Z 356 | |     };
2020-01-18T07:05:32.6901793Z 357 | | }
2020-01-18T07:05:32.6902003Z     | | -
2020-01-18T07:05:32.6902204Z     | |_|
2020-01-18T07:05:32.6902204Z     | |_|
2020-01-18T07:05:32.6902441Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6902665Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.6903402Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.6903780Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:05:32.6904186Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.6904441Z     |
2020-01-18T07:05:32.6904793Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.6904862Z 
2020-01-18T07:05:32.7031381Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&mut ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T07:05:32.7031743Z    --> src/libcore/ops/function.rs:336:13
2020-01-18T07:05:32.7031974Z     |
2020-01-18T07:05:32.7032296Z 269 | /     impl<A, F: ?Sized> FnMut<A> for &mut F
2020-01-18T07:05:32.7033211Z 271 | |         F: FnMut<A>,
2020-01-18T07:05:32.7033512Z 272 | |     {
2020-01-18T07:05:32.7033777Z ...   |
2020-01-18T07:05:32.7034064Z 275 | |         }
2020-01-18T07:05:32.7034064Z 275 | |         }
2020-01-18T07:05:32.7034350Z 276 | |     }
2020-01-18T07:05:32.7034687Z     | |_____- first implementation here
2020-01-18T07:05:32.7034891Z ...
2020-01-18T07:05:32.7035174Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.7035587Z     |  _-
2020-01-18T07:05:32.7035873Z     | |_|
2020-01-18T07:05:32.7036109Z     | |
2020-01-18T07:05:32.7036475Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.7036801Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.7037169Z     |  _________-
2020-01-18T07:05:32.7037512Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.7038083Z ...
2020-01-18T07:05:32.7038083Z ...
2020-01-18T07:05:32.7038412Z 336 | /             impl<A, F: ?Sized> FnMut<A> for &mut $z<F>
2020-01-18T07:05:32.7038988Z 338 |                   F: FnMut<A>,
2020-01-18T07:05:32.7039277Z 339 |               {
2020-01-18T07:05:32.7039478Z ...
2020-01-18T07:05:32.7039760Z 342 |                   }
2020-01-18T07:05:32.7039760Z 342 |                   }
2020-01-18T07:05:32.7040056Z 343 | |             }
2020-01-18T07:05:32.7040521Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.7042153Z 356 | |     };
2020-01-18T07:05:32.7043566Z 357 | | }
2020-01-18T07:05:32.7043890Z     | | -
2020-01-18T07:05:32.7044151Z     | |_|
2020-01-18T07:05:32.7044151Z     | |_|
2020-01-18T07:05:32.7044471Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7044772Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7045102Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.7045480Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:05:32.7045890Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.7046153Z     |
2020-01-18T07:05:32.7046507Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.7046553Z 
2020-01-18T07:05:32.7172972Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T07:05:32.7173960Z    --> src/libcore/ops/function.rs:324:13
2020-01-18T07:05:32.7174275Z     |
2020-01-18T07:05:32.7174595Z 257 | /     impl<A, F: ?Sized> FnOnce<A> for &F
2020-01-18T07:05:32.7175216Z 259 | |         F: Fn<A>,
2020-01-18T07:05:32.7175658Z 260 | |     {
2020-01-18T07:05:32.7175900Z ...   |
2020-01-18T07:05:32.7176221Z 265 | |         }
2020-01-18T07:05:32.7176221Z 265 | |         }
2020-01-18T07:05:32.7177155Z 266 | |     }
2020-01-18T07:05:32.7177623Z     | |_____- first implementation here
2020-01-18T07:05:32.7177792Z ...
2020-01-18T07:05:32.7178017Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.7178230Z     |  _-
2020-01-18T07:05:32.7178435Z     | |_|
2020-01-18T07:05:32.7178620Z     | |
2020-01-18T07:05:32.7178907Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.7179173Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.7179384Z     |  _________-
2020-01-18T07:05:32.7179651Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.7180095Z ...
2020-01-18T07:05:32.7180095Z ...
2020-01-18T07:05:32.7180370Z 324 | /             impl<A, F: ?Sized> FnOnce<A> for &$z<F>
2020-01-18T07:05:32.7180817Z 326 |                   F: Fn<A>,
2020-01-18T07:05:32.7181051Z 327 |               {
2020-01-18T07:05:32.7181212Z ...
2020-01-18T07:05:32.7181421Z 332 |                   }
2020-01-18T07:05:32.7181421Z 332 |                   }
2020-01-18T07:05:32.7181670Z 333 | |             }
2020-01-18T07:05:32.7181953Z     | |_____________^ conflicting implementation for `&ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.7182366Z 356 | |     };
2020-01-18T07:05:32.7183242Z 357 | | }
2020-01-18T07:05:32.7183650Z     | | -
2020-01-18T07:05:32.7183935Z     | |_|
2020-01-18T07:05:32.7183935Z     | |_|
2020-01-18T07:05:32.7184239Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7184551Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7184900Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.7185387Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.7185637Z     |
2020-01-18T07:05:32.7185994Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.7186039Z 
2020-01-18T07:05:32.7289297Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&mut ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T07:05:32.7289550Z    --> src/libcore/ops/function.rs:346:13
2020-01-18T07:05:32.7289930Z     |
2020-01-18T07:05:32.7290200Z 279 | /     impl<A, F: ?Sized> FnOnce<A> for &mut F
2020-01-18T07:05:32.7290722Z 281 | |         F: FnMut<A>,
2020-01-18T07:05:32.7290965Z 282 | |     {
2020-01-18T07:05:32.7291164Z ...   |
2020-01-18T07:05:32.7291416Z 286 | |         }
2020-01-18T07:05:32.7291416Z 286 | |         }
2020-01-18T07:05:32.7291651Z 287 | |     }
2020-01-18T07:05:32.7291898Z     | |_____- first implementation here
2020-01-18T07:05:32.7292093Z ...
2020-01-18T07:05:32.7292324Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.7292531Z     |  _-
2020-01-18T07:05:32.7292744Z     | |_|
2020-01-18T07:05:32.7293416Z     | |
2020-01-18T07:05:32.7293775Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.7294124Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.7294388Z     |  _________-
2020-01-18T07:05:32.7294722Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.7295283Z ...
2020-01-18T07:05:32.7295283Z ...
2020-01-18T07:05:32.7295774Z 346 | /             impl<A, F: ?Sized> FnOnce<A> for &mut $z<F>
2020-01-18T07:05:32.7296392Z 348 |                   F: FnMut<A>,
2020-01-18T07:05:32.7296815Z 349 |               {
2020-01-18T07:05:32.7296975Z ...
2020-01-18T07:05:32.7297184Z 353 |                   }
2020-01-18T07:05:32.7297184Z 353 |                   }
2020-01-18T07:05:32.7297530Z 354 | |             }
2020-01-18T07:05:32.7297815Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.7298284Z 356 | |     };
2020-01-18T07:05:32.7298507Z 357 | | }
2020-01-18T07:05:32.7298723Z     | | -
2020-01-18T07:05:32.7298931Z     | |_|
2020-01-18T07:05:32.7298931Z     | |_|
2020-01-18T07:05:32.7299181Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7299431Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7299714Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.7300033Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.7300232Z     |
2020-01-18T07:05:32.7300510Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T07:05:32.7300547Z 
2020-01-18T07:05:32.7389071Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T07:05:32.7389338Z    --> src/libcore/ops/function.rs:324:13
2020-01-18T07:05:32.7389547Z     |
2020-01-18T07:05:32.7389834Z 257 | /     impl<A, F: ?Sized> FnOnce<A> for &F
2020-01-18T07:05:32.7390356Z 259 | |         F: Fn<A>,
2020-01-18T07:05:32.7390603Z 260 | |     {
2020-01-18T07:05:32.7390805Z ...   |
2020-01-18T07:05:32.7391074Z 265 | |         }
2020-01-18T07:05:32.7391074Z 265 | |         }
2020-01-18T07:05:32.7391317Z 266 | |     }
2020-01-18T07:05:32.7391572Z     | |_____- first implementation here
2020-01-18T07:05:32.7391889Z ...
2020-01-18T07:05:32.7392176Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.7392387Z     |  _-
2020-01-18T07:05:32.7392613Z     | |_|
2020-01-18T07:05:32.7393209Z     | |
2020-01-18T07:05:32.7393600Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.7394098Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.7394360Z     |  _________-
2020-01-18T07:05:32.7394684Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.7395253Z ...
2020-01-18T07:05:32.7395253Z ...
2020-01-18T07:05:32.7395591Z 324 | /             impl<A, F: ?Sized> FnOnce<A> for &$z<F>
2020-01-18T07:05:32.7396160Z 326 |                   F: Fn<A>,
2020-01-18T07:05:32.7396442Z 327 |               {
2020-01-18T07:05:32.7396788Z ...
2020-01-18T07:05:32.7397026Z 332 |                   }
2020-01-18T07:05:32.7397026Z 332 |                   }
2020-01-18T07:05:32.7397465Z 333 | |             }
2020-01-18T07:05:32.7397921Z     | |_____________^ conflicting implementation for `&ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.7398333Z 356 | |     };
2020-01-18T07:05:32.7398563Z 357 | | }
2020-01-18T07:05:32.7398794Z     | | -
2020-01-18T07:05:32.7399011Z     | |_|
2020-01-18T07:05:32.7399011Z     | |_|
2020-01-18T07:05:32.7399260Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7399508Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7399790Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.7400071Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:05:32.7400413Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.7400613Z     |
2020-01-18T07:05:32.7401049Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.7401108Z 
2020-01-18T07:05:32.7489782Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&mut ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T07:05:32.7490232Z    --> src/libcore/ops/function.rs:346:13
2020-01-18T07:05:32.7490437Z     |
2020-01-18T07:05:32.7490691Z 279 | /     impl<A, F: ?Sized> FnOnce<A> for &mut F
2020-01-18T07:05:32.7491468Z 281 | |         F: FnMut<A>,
2020-01-18T07:05:32.7491702Z 282 | |     {
2020-01-18T07:05:32.7491921Z ...   |
2020-01-18T07:05:32.7492164Z 286 | |         }
2020-01-18T07:05:32.7492164Z 286 | |         }
2020-01-18T07:05:32.7492397Z 287 | |     }
2020-01-18T07:05:32.7492659Z     | |_____- first implementation here
2020-01-18T07:05:32.7492833Z ...
2020-01-18T07:05:32.7493066Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T07:05:32.7494066Z     |  _-
2020-01-18T07:05:32.7494315Z     | |_|
2020-01-18T07:05:32.7494551Z     | |
2020-01-18T07:05:32.7494924Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:05:32.7495257Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:05:32.7495518Z     |  _________-
2020-01-18T07:05:32.7495855Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T07:05:32.7496410Z ...
2020-01-18T07:05:32.7496410Z ...
2020-01-18T07:05:32.7496760Z 346 | /             impl<A, F: ?Sized> FnOnce<A> for &mut $z<F>
2020-01-18T07:05:32.7497481Z 348 |                   F: FnMut<A>,
2020-01-18T07:05:32.7497703Z 349 |               {
2020-01-18T07:05:32.7497866Z ...
2020-01-18T07:05:32.7498098Z 353 |                   }
2020-01-18T07:05:32.7498098Z 353 |                   }
2020-01-18T07:05:32.7498337Z 354 | |             }
2020-01-18T07:05:32.7498638Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.7499197Z 356 | |     };
2020-01-18T07:05:32.7499459Z 357 | | }
2020-01-18T07:05:32.7499699Z     | | -
2020-01-18T07:05:32.7499898Z     | |_|
2020-01-18T07:05:32.7499898Z     | |_|
2020-01-18T07:05:32.7500296Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7500545Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:05:32.7500886Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:05:32.7501167Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:05:32.7501506Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T07:05:32.7501686Z     |
2020-01-18T07:05:32.7501979Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T07:05:32.8973976Z error: aborting due to 12 previous errors
2020-01-18T07:05:32.8974082Z 
2020-01-18T07:05:32.8981255Z For more information about this error, try `rustc --explain E0119`.
2020-01-18T07:05:32.9131434Z error: could not compile `core`.
---
2020-01-18T07:05:34.4560427Z   local time: Sat Jan 18 07:05:34 UTC 2020
2020-01-18T07:05:34.7209404Z   network time: Sat, 18 Jan 2020 07:05:34 GMT
2020-01-18T07:05:34.7210795Z == end clock drift check ==
2020-01-18T07:05:42.4881330Z 
2020-01-18T07:05:42.4998928Z ##[error]Bash exited with code '1'.
2020-01-18T07:05:42.5011466Z ##[section]Finishing: Run build
2020-01-18T07:05:42.5025809Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:05:42.5027760Z Task         : Get sources
2020-01-18T07:05:42.5027800Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:05:42.5027856Z Version      : 1.0.0
2020-01-18T07:05:42.5027909Z Author       : Microsoft
2020-01-18T07:05:42.5027909Z Author       : Microsoft
2020-01-18T07:05:42.5027947Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T07:05:42.5028005Z ==============================================================================
2020-01-18T07:05:42.9152053Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T07:05:42.9199409Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:05:42.9306460Z Cleaning up task key
2020-01-18T07:05:42.9307486Z Start cleaning up orphan processes.
2020-01-18T07:05:42.9465195Z Terminate orphan process: pid (3320) (python)
2020-01-18T07:05:42.9805120Z ##[section]Finishing: Finalize Job
