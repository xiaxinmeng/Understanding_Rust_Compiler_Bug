plain
2020-01-18T06:48:50.6430646Z ========================== Starting Command Output ===========================
2020-01-18T06:48:50.6432491Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8c67ef02-5ef7-41bc-a525-0e75003444a6.sh
2020-01-18T06:48:50.6432593Z 
2020-01-18T06:48:50.6435821Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T06:48:50.6442715Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:48:50.6444600Z Task         : Get sources
2020-01-18T06:48:50.6444700Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T06:48:50.6444736Z Version      : 1.0.0
2020-01-18T06:48:50.6444773Z Author       : Microsoft
---
2020-01-18T06:48:52.3037541Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T06:48:52.3047772Z ##[command]git config gc.auto 0
2020-01-18T06:48:52.3053287Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T06:48:52.3058207Z ##[command]git config --get-all http.proxy
2020-01-18T06:48:52.3064688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T06:52:47.0911658Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T06:52:48.6093538Z error: expected unsuffixed literal or identifier, found `fn_not_impl`
2020-01-18T06:52:48.6097116Z    --> src/libcore/ops/function.rs:358:41
2020-01-18T06:52:48.6098272Z     |
2020-01-18T06:52:48.6101271Z 358 | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:48.6101706Z 
2020-01-18T06:52:48.6126014Z error: expected unsuffixed literal or identifier, found `fn_neg_impl`
2020-01-18T06:52:48.6126839Z    --> src/libcore/ops/function.rs:359:41
2020-01-18T06:52:48.6127378Z     |
2020-01-18T06:52:48.6127378Z     |
2020-01-18T06:52:48.6128005Z 359 | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:52:48.6128786Z 
2020-01-18T06:52:48.6128786Z 
2020-01-18T06:52:51.4945722Z error[E0119]: conflicting implementations of trait `ops::function::Fn<_>` for type `&ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T06:52:51.4946480Z    --> src/libcore/ops/function.rs:304:13
2020-01-18T06:52:51.4946729Z     |
2020-01-18T06:52:51.4947346Z 237 | /     impl<A, F: ?Sized> Fn<A> for &F
2020-01-18T06:52:51.4947976Z 239 | |         F: Fn<A>,
2020-01-18T06:52:51.4948602Z 240 | |     {
2020-01-18T06:52:51.4948864Z ...   |
2020-01-18T06:52:51.4949157Z 243 | |         }
2020-01-18T06:52:51.4949157Z 243 | |         }
2020-01-18T06:52:51.4950003Z 244 | |     }
2020-01-18T06:52:51.4950323Z     | |_____- first implementation here
2020-01-18T06:52:51.4950915Z ...
2020-01-18T06:52:51.4951227Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.4951674Z     |  _-
2020-01-18T06:52:51.4952033Z     | |_|
2020-01-18T06:52:51.4952285Z     | |
2020-01-18T06:52:51.4953008Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.4953391Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.4953993Z     |  _________-
2020-01-18T06:52:51.4954319Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.4955228Z ...
2020-01-18T06:52:51.4955228Z ...
2020-01-18T06:52:51.4955553Z 304 | /             impl<A, F: ?Sized> Fn<A> for &$z<F>
2020-01-18T06:52:51.4956165Z 306 |                   F: Fn<A>,
2020-01-18T06:52:51.4956448Z 307 |               {
2020-01-18T06:52:51.4956683Z ...
2020-01-18T06:52:51.4956957Z 310 |                   }
2020-01-18T06:52:51.4956957Z 310 |                   }
2020-01-18T06:52:51.4957255Z 311 | |             }
2020-01-18T06:52:51.4957753Z     | |_____________^ conflicting implementation for `&ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.4958237Z 356 | |     };
2020-01-18T06:52:51.4958538Z 357 | | }
2020-01-18T06:52:51.4958924Z     | | -
2020-01-18T06:52:51.4959221Z     | |_|
2020-01-18T06:52:51.4959221Z     | |_|
2020-01-18T06:52:51.4959528Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.4959820Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.4960168Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.4960560Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.4960788Z     |
2020-01-18T06:52:51.4961265Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.4961316Z 
2020-01-18T06:52:51.4961656Z error[E0119]: conflicting implementations of trait `ops::function::Fn<_>` for type `&ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T06:52:51.4961924Z    --> src/libcore/ops/function.rs:304:13
2020-01-18T06:52:51.4962139Z     |
2020-01-18T06:52:51.4962461Z 237 | /     impl<A, F: ?Sized> Fn<A> for &F
2020-01-18T06:52:51.4963052Z 239 | |         F: Fn<A>,
2020-01-18T06:52:51.4963491Z 240 | |     {
2020-01-18T06:52:51.4963793Z ...   |
2020-01-18T06:52:51.4964095Z 243 | |         }
2020-01-18T06:52:51.4964095Z 243 | |         }
2020-01-18T06:52:51.4964409Z 244 | |     }
2020-01-18T06:52:51.4964718Z     | |_____- first implementation here
2020-01-18T06:52:51.4964930Z ...
2020-01-18T06:52:51.4965242Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.4965501Z     |  _-
2020-01-18T06:52:51.4965748Z     | |_|
2020-01-18T06:52:51.4966024Z     | |
2020-01-18T06:52:51.4966523Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.4967001Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.4967283Z     |  _________-
2020-01-18T06:52:51.4967594Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.4968154Z ...
2020-01-18T06:52:51.4968154Z ...
2020-01-18T06:52:51.4968575Z 304 | /             impl<A, F: ?Sized> Fn<A> for &$z<F>
2020-01-18T06:52:51.4969187Z 306 |                   F: Fn<A>,
2020-01-18T06:52:51.4969452Z 307 |               {
2020-01-18T06:52:51.4969675Z ...
2020-01-18T06:52:51.4969939Z 310 |                   }
2020-01-18T06:52:51.4969939Z 310 |                   }
2020-01-18T06:52:51.4970225Z 311 | |             }
2020-01-18T06:52:51.4970591Z     | |_____________^ conflicting implementation for `&ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.4971195Z 356 | |     };
2020-01-18T06:52:51.4971504Z 357 | | }
2020-01-18T06:52:51.4971775Z     | | -
2020-01-18T06:52:51.4972031Z     | |_|
2020-01-18T06:52:51.4972031Z     | |_|
2020-01-18T06:52:51.4972335Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.4972626Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.4972963Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.4973317Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:52:51.4973723Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.4973956Z     |
2020-01-18T06:52:51.4974310Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.4974353Z 
2020-01-18T06:52:51.5052774Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T06:52:51.5053429Z    --> src/libcore/ops/function.rs:314:13
2020-01-18T06:52:51.5053708Z     |
2020-01-18T06:52:51.5054313Z 247 | /     impl<A, F: ?Sized> FnMut<A> for &F
2020-01-18T06:52:51.5055312Z 249 | |         F: Fn<A>,
2020-01-18T06:52:51.5055641Z 250 | |     {
2020-01-18T06:52:51.5056273Z ...   |
2020-01-18T06:52:51.5056579Z 253 | |         }
2020-01-18T06:52:51.5056579Z 253 | |         }
2020-01-18T06:52:51.5057152Z 254 | |     }
2020-01-18T06:52:51.5057499Z     | |_____- first implementation here
2020-01-18T06:52:51.5058194Z ...
2020-01-18T06:52:51.5058870Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5059392Z     |  _-
2020-01-18T06:52:51.5059713Z     | |_|
2020-01-18T06:52:51.5059960Z     | |
2020-01-18T06:52:51.5060636Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5061326Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5061644Z     |  _________-
2020-01-18T06:52:51.5062469Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5063389Z ...
2020-01-18T06:52:51.5063389Z ...
2020-01-18T06:52:51.5064011Z 314 | /             impl<A, F: ?Sized> FnMut<A> for &$z<F>
2020-01-18T06:52:51.5064980Z 316 |                   F: Fn<A>,
2020-01-18T06:52:51.5065583Z 317 |               {
2020-01-18T06:52:51.5065828Z ...
2020-01-18T06:52:51.5066371Z 320 |                   }
2020-01-18T06:52:51.5066371Z 320 |                   }
2020-01-18T06:52:51.5066718Z 321 | |             }
2020-01-18T06:52:51.5067077Z     | |_____________^ conflicting implementation for `&ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5067961Z 356 | |     };
2020-01-18T06:52:51.5068592Z 357 | | }
2020-01-18T06:52:51.5068907Z     | | -
2020-01-18T06:52:51.5069510Z     | |_|
2020-01-18T06:52:51.5069510Z     | |_|
2020-01-18T06:52:51.5070097Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5070472Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5071122Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5071557Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5072103Z     |
2020-01-18T06:52:51.5072466Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5072833Z 
2020-01-18T06:52:51.5129727Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&mut ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T06:52:51.5130428Z    --> src/libcore/ops/function.rs:336:13
2020-01-18T06:52:51.5131031Z     |
2020-01-18T06:52:51.5131376Z 269 | /     impl<A, F: ?Sized> FnMut<A> for &mut F
2020-01-18T06:52:51.5132619Z 271 | |         F: FnMut<A>,
2020-01-18T06:52:51.5132959Z 272 | |     {
2020-01-18T06:52:51.5133231Z ...   |
2020-01-18T06:52:51.5134064Z 275 | |         }
2020-01-18T06:52:51.5134064Z 275 | |         }
2020-01-18T06:52:51.5134645Z 276 | |     }
2020-01-18T06:52:51.5135296Z     | |_____- first implementation here
2020-01-18T06:52:51.5135567Z ...
2020-01-18T06:52:51.5136201Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5136523Z     |  _-
2020-01-18T06:52:51.5137041Z     | |_|
2020-01-18T06:52:51.5137345Z     | |
2020-01-18T06:52:51.5137716Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5138377Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5138665Z     |  _________-
2020-01-18T06:52:51.5139339Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5140193Z ...
2020-01-18T06:52:51.5140193Z ...
2020-01-18T06:52:51.5140573Z 336 | /             impl<A, F: ?Sized> FnMut<A> for &mut $z<F>
2020-01-18T06:52:51.5141500Z 338 |                   F: FnMut<A>,
2020-01-18T06:52:51.5141818Z 339 |               {
2020-01-18T06:52:51.5142029Z ...
2020-01-18T06:52:51.5142682Z 342 |                   }
2020-01-18T06:52:51.5142682Z 342 |                   }
2020-01-18T06:52:51.5142989Z 343 | |             }
2020-01-18T06:52:51.5143613Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5144400Z 356 | |     };
2020-01-18T06:52:51.5144809Z 357 | | }
2020-01-18T06:52:51.5145526Z     | | -
2020-01-18T06:52:51.5145885Z     | |_|
2020-01-18T06:52:51.5145885Z     | |_|
2020-01-18T06:52:51.5146559Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5147199Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5147603Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5148059Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5148790Z     |
2020-01-18T06:52:51.5149214Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5149269Z 
2020-01-18T06:52:51.5241310Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T06:52:51.5241750Z    --> src/libcore/ops/function.rs:314:13
2020-01-18T06:52:51.5241986Z     |
2020-01-18T06:52:51.5242324Z 247 | /     impl<A, F: ?Sized> FnMut<A> for &F
2020-01-18T06:52:51.5242931Z 249 | |         F: Fn<A>,
2020-01-18T06:52:51.5243279Z 250 | |     {
2020-01-18T06:52:51.5243529Z ...   |
2020-01-18T06:52:51.5243821Z 253 | |         }
2020-01-18T06:52:51.5243821Z 253 | |         }
2020-01-18T06:52:51.5244134Z 254 | |     }
2020-01-18T06:52:51.5244443Z     | |_____- first implementation here
2020-01-18T06:52:51.5244654Z ...
2020-01-18T06:52:51.5244969Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5245228Z     |  _-
2020-01-18T06:52:51.5245485Z     | |_|
2020-01-18T06:52:51.5245759Z     | |
2020-01-18T06:52:51.5246106Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5246732Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5247038Z     |  _________-
2020-01-18T06:52:51.5247361Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5247954Z ...
2020-01-18T06:52:51.5247954Z ...
2020-01-18T06:52:51.5248435Z 314 | /             impl<A, F: ?Sized> FnMut<A> for &$z<F>
2020-01-18T06:52:51.5249084Z 316 |                   F: Fn<A>,
2020-01-18T06:52:51.5249358Z 317 |               {
2020-01-18T06:52:51.5249588Z ...
2020-01-18T06:52:51.5249859Z 320 |                   }
2020-01-18T06:52:51.5249859Z 320 |                   }
2020-01-18T06:52:51.5250151Z 321 | |             }
2020-01-18T06:52:51.5250531Z     | |_____________^ conflicting implementation for `&ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5251195Z 356 | |     };
2020-01-18T06:52:51.5251619Z 357 | | }
2020-01-18T06:52:51.5251888Z     | | -
2020-01-18T06:52:51.5252129Z     | |_|
2020-01-18T06:52:51.5252129Z     | |_|
2020-01-18T06:52:51.5252572Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5252874Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5253211Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5253581Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:52:51.5253980Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5254237Z     |
2020-01-18T06:52:51.5254588Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5254643Z 
2020-01-18T06:52:51.5255005Z error[E0119]: conflicting implementations of trait `ops::function::FnMut<_>` for type `&mut ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T06:52:51.5255278Z    --> src/libcore/ops/function.rs:336:13
2020-01-18T06:52:51.5255524Z     |
2020-01-18T06:52:51.5255861Z 269 | /     impl<A, F: ?Sized> FnMut<A> for &mut F
2020-01-18T06:52:51.5256484Z 271 | |         F: FnMut<A>,
2020-01-18T06:52:51.5256783Z 272 | |     {
2020-01-18T06:52:51.5257030Z ...   |
2020-01-18T06:52:51.5257337Z 275 | |         }
2020-01-18T06:52:51.5257337Z 275 | |         }
2020-01-18T06:52:51.5257632Z 276 | |     }
2020-01-18T06:52:51.5258032Z     | |_____- first implementation here
2020-01-18T06:52:51.5258305Z ...
2020-01-18T06:52:51.5258595Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5258854Z     |  _-
2020-01-18T06:52:51.5259124Z     | |_|
2020-01-18T06:52:51.5259368Z     | |
2020-01-18T06:52:51.5259713Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5260063Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5260553Z     |  _________-
2020-01-18T06:52:51.5260874Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5261438Z ...
2020-01-18T06:52:51.5261438Z ...
2020-01-18T06:52:51.5261774Z 336 | /             impl<A, F: ?Sized> FnMut<A> for &mut $z<F>
2020-01-18T06:52:51.5262451Z 338 |                   F: FnMut<A>,
2020-01-18T06:52:51.5262742Z 339 |               {
2020-01-18T06:52:51.5262970Z ...
2020-01-18T06:52:51.5263251Z 342 |                   }
2020-01-18T06:52:51.5263251Z 342 |                   }
2020-01-18T06:52:51.5263565Z 343 | |             }
2020-01-18T06:52:51.5263931Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5264463Z 356 | |     };
2020-01-18T06:52:51.5264756Z 357 | | }
2020-01-18T06:52:51.5265051Z     | | -
2020-01-18T06:52:51.5265316Z     | |_|
2020-01-18T06:52:51.5265316Z     | |_|
2020-01-18T06:52:51.5265633Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5265958Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5266286Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5266632Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:52:51.5267055Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5267291Z     |
2020-01-18T06:52:51.5267758Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5267812Z 
2020-01-18T06:52:51.5308683Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T06:52:51.5309004Z    --> src/libcore/ops/function.rs:324:13
2020-01-18T06:52:51.5309310Z     |
2020-01-18T06:52:51.5309625Z 257 | /     impl<A, F: ?Sized> FnOnce<A> for &F
2020-01-18T06:52:51.5310227Z 259 | |         F: Fn<A>,
2020-01-18T06:52:51.5310701Z 260 | |     {
2020-01-18T06:52:51.5310965Z ...   |
2020-01-18T06:52:51.5311254Z 265 | |         }
2020-01-18T06:52:51.5311254Z 265 | |         }
2020-01-18T06:52:51.5311534Z 266 | |     }
2020-01-18T06:52:51.5311850Z     | |_____- first implementation here
2020-01-18T06:52:51.5312062Z ...
2020-01-18T06:52:51.5312345Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5312610Z     |  _-
2020-01-18T06:52:51.5312856Z     | |_|
2020-01-18T06:52:51.5313235Z     | |
2020-01-18T06:52:51.5314099Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5316171Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5317083Z     |  _________-
2020-01-18T06:52:51.5317443Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5318033Z ...
2020-01-18T06:52:51.5318033Z ...
2020-01-18T06:52:51.5318399Z 324 | /             impl<A, F: ?Sized> FnOnce<A> for &$z<F>
2020-01-18T06:52:51.5318998Z 326 |                   F: Fn<A>,
2020-01-18T06:52:51.5319273Z 327 |               {
2020-01-18T06:52:51.5319483Z ...
2020-01-18T06:52:51.5319884Z 332 |                   }
2020-01-18T06:52:51.5319884Z 332 |                   }
2020-01-18T06:52:51.5320684Z 333 | |             }
2020-01-18T06:52:51.5321190Z     | |_____________^ conflicting implementation for `&ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5321854Z 356 | |     };
2020-01-18T06:52:51.5322313Z 357 | | }
2020-01-18T06:52:51.5322623Z     | | -
2020-01-18T06:52:51.5322873Z     | |_|
2020-01-18T06:52:51.5322873Z     | |_|
2020-01-18T06:52:51.5323204Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5323509Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5323857Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5326262Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5326710Z     |
2020-01-18T06:52:51.5327713Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5335624Z 
2020-01-18T06:52:51.5446163Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&mut ops::function::fn_not_impl::FnNot<_>`:
2020-01-18T06:52:51.5446679Z    --> src/libcore/ops/function.rs:346:13
2020-01-18T06:52:51.5446903Z     |
2020-01-18T06:52:51.5447223Z 279 | /     impl<A, F: ?Sized> FnOnce<A> for &mut F
2020-01-18T06:52:51.5447826Z 281 | |         F: FnMut<A>,
2020-01-18T06:52:51.5448117Z 282 | |     {
2020-01-18T06:52:51.5448356Z ...   |
2020-01-18T06:52:51.5448627Z 286 | |         }
2020-01-18T06:52:51.5448627Z 286 | |         }
2020-01-18T06:52:51.5448919Z 287 | |     }
2020-01-18T06:52:51.5449204Z     | |_____- first implementation here
2020-01-18T06:52:51.5449405Z ...
2020-01-18T06:52:51.5449712Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5449954Z     |  _-
2020-01-18T06:52:51.5450182Z     | |_|
2020-01-18T06:52:51.5450429Z     | |
2020-01-18T06:52:51.5450748Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5451053Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5451331Z     |  _________-
2020-01-18T06:52:51.5451631Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5452361Z ...
2020-01-18T06:52:51.5452361Z ...
2020-01-18T06:52:51.5452671Z 346 | /             impl<A, F: ?Sized> FnOnce<A> for &mut $z<F>
2020-01-18T06:52:51.5453229Z 348 |                   F: FnMut<A>,
2020-01-18T06:52:51.5453486Z 349 |               {
2020-01-18T06:52:51.5453701Z ...
2020-01-18T06:52:51.5453955Z 353 |                   }
2020-01-18T06:52:51.5453955Z 353 |                   }
2020-01-18T06:52:51.5454353Z 354 | |             }
2020-01-18T06:52:51.5454717Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5455270Z 356 | |     };
2020-01-18T06:52:51.5455547Z 357 | | }
2020-01-18T06:52:51.5455810Z     | | -
2020-01-18T06:52:51.5456063Z     | |_|
2020-01-18T06:52:51.5456063Z     | |_|
2020-01-18T06:52:51.5456351Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5456646Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5456989Z 358 | | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5457361Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5457603Z     |
2020-01-18T06:52:51.5457931Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_not_impl::FnNot<_>`
2020-01-18T06:52:51.5466232Z 
2020-01-18T06:52:51.5466946Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T06:52:51.5467367Z    --> src/libcore/ops/function.rs:324:13
2020-01-18T06:52:51.5467582Z     |
2020-01-18T06:52:51.5467906Z 257 | /     impl<A, F: ?Sized> FnOnce<A> for &F
2020-01-18T06:52:51.5468482Z 259 | |         F: Fn<A>,
2020-01-18T06:52:51.5468759Z 260 | |     {
2020-01-18T06:52:51.5468990Z ...   |
2020-01-18T06:52:51.5469275Z 265 | |         }
2020-01-18T06:52:51.5469275Z 265 | |         }
2020-01-18T06:52:51.5469679Z 266 | |     }
2020-01-18T06:52:51.5470934Z     | |_____- first implementation here
2020-01-18T06:52:51.5471242Z ...
2020-01-18T06:52:51.5471591Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5471852Z     |  _-
2020-01-18T06:52:51.5472115Z     | |_|
2020-01-18T06:52:51.5472365Z     | |
2020-01-18T06:52:51.5472715Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5473245Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5473517Z     |  _________-
2020-01-18T06:52:51.5474080Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5474773Z ...
2020-01-18T06:52:51.5474773Z ...
2020-01-18T06:52:51.5475086Z 324 | /             impl<A, F: ?Sized> FnOnce<A> for &$z<F>
2020-01-18T06:52:51.5475653Z 326 |                   F: Fn<A>,
2020-01-18T06:52:51.5475949Z 327 |               {
2020-01-18T06:52:51.5476157Z ...
2020-01-18T06:52:51.5476417Z 332 |                   }
2020-01-18T06:52:51.5476417Z 332 |                   }
2020-01-18T06:52:51.5476716Z 333 | |             }
2020-01-18T06:52:51.5477061Z     | |_____________^ conflicting implementation for `&ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5477557Z 356 | |     };
2020-01-18T06:52:51.5477839Z 357 | | }
2020-01-18T06:52:51.5478243Z     | | -
2020-01-18T06:52:51.5478494Z     | |_|
2020-01-18T06:52:51.5478494Z     | |_|
2020-01-18T06:52:51.5478786Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5479064Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5479384Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5479708Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:52:51.5480091Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5480400Z     |
2020-01-18T06:52:51.5480755Z     = note: downstream crates may implement trait `ops::function::Fn<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5480797Z 
2020-01-18T06:52:51.5573829Z error[E0119]: conflicting implementations of trait `ops::function::FnOnce<_>` for type `&mut ops::function::fn_neg_impl::FnNeg<_>`:
2020-01-18T06:52:51.5574131Z    --> src/libcore/ops/function.rs:346:13
2020-01-18T06:52:51.5574379Z     |
2020-01-18T06:52:51.5574684Z 279 | /     impl<A, F: ?Sized> FnOnce<A> for &mut F
2020-01-18T06:52:51.5575470Z 281 | |         F: FnMut<A>,
2020-01-18T06:52:51.5575747Z 282 | |     {
2020-01-18T06:52:51.5575978Z ...   |
2020-01-18T06:52:51.5576272Z 286 | |         }
2020-01-18T06:52:51.5576272Z 286 | |         }
2020-01-18T06:52:51.5576542Z 287 | |     }
2020-01-18T06:52:51.5576845Z     | |_____- first implementation here
2020-01-18T06:52:51.5577047Z ...
2020-01-18T06:52:51.5577318Z 290 |   macro_rules! gen_fn_struct_unopt {
2020-01-18T06:52:51.5577585Z     |  _-
2020-01-18T06:52:51.5577828Z     | |_|
2020-01-18T06:52:51.5578056Z     | |
2020-01-18T06:52:51.5578395Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:52:51.5578705Z 292 |           gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T06:52:51.5578954Z     |  _________-
2020-01-18T06:52:51.5579273Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:52:51.5579813Z ...
2020-01-18T06:52:51.5579813Z ...
2020-01-18T06:52:51.5580145Z 346 | /             impl<A, F: ?Sized> FnOnce<A> for &mut $z<F>
2020-01-18T06:52:51.5580678Z 348 |                   F: FnMut<A>,
2020-01-18T06:52:51.5580960Z 349 |               {
2020-01-18T06:52:51.5581155Z ...
2020-01-18T06:52:51.5581407Z 353 |                   }
2020-01-18T06:52:51.5581407Z 353 |                   }
2020-01-18T06:52:51.5581701Z 354 | |             }
2020-01-18T06:52:51.5582135Z     | |_____________^ conflicting implementation for `&mut ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.5582726Z 356 | |     };
2020-01-18T06:52:51.5582992Z 357 | | }
2020-01-18T06:52:51.5583269Z     | | -
2020-01-18T06:52:51.5583507Z     | |_|
2020-01-18T06:52:51.5583507Z     | |_|
2020-01-18T06:52:51.5583793Z     | |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5584093Z     |   in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:52:51.5584506Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:52:51.5584834Z 359 | | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:52:51.5585223Z     | | -------------------------------------------------------------- in this macro invocation
2020-01-18T06:52:51.5585439Z     |
2020-01-18T06:52:51.5585784Z     = note: downstream crates may implement trait `ops::function::FnMut<_>` for type `ops::function::fn_neg_impl::FnNeg<_>`
2020-01-18T06:52:51.6794818Z error: aborting due to 12 previous errors
2020-01-18T06:52:51.6794942Z 
2020-01-18T06:52:51.6800275Z For more information about this error, try `rustc --explain E0119`.
2020-01-18T06:52:51.6905636Z error: could not compile `core`.
---
2020-01-18T06:52:53.2574989Z   local time: Sat Jan 18 06:52:53 UTC 2020
2020-01-18T06:52:53.7759622Z   network time: Sat, 18 Jan 2020 06:52:53 GMT
2020-01-18T06:52:53.7769270Z == end clock drift check ==
2020-01-18T06:53:01.5897765Z 
2020-01-18T06:53:01.6011127Z ##[error]Bash exited with code '1'.
2020-01-18T06:53:01.6025454Z ##[section]Finishing: Run build
2020-01-18T06:53:01.6041768Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:53:01.6043804Z Task         : Get sources
2020-01-18T06:53:01.6043857Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T06:53:01.6043928Z Version      : 1.0.0
2020-01-18T06:53:01.6043975Z Author       : Microsoft
2020-01-18T06:53:01.6043975Z Author       : Microsoft
2020-01-18T06:53:01.6044042Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T06:53:01.6044115Z ==============================================================================
2020-01-18T06:53:02.1676416Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T06:53:02.1718965Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:53:02.1844552Z Cleaning up task key
2020-01-18T06:53:02.1845431Z Start cleaning up orphan processes.
2020-01-18T06:53:02.1974507Z Terminate orphan process: pid (3462) (python)
2020-01-18T06:53:02.2189516Z ##[section]Finishing: Finalize Job
