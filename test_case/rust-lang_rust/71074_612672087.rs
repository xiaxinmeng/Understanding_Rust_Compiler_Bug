plain
2020-04-12T20:07:16.1808045Z ========================== Starting Command Output ===========================
2020-04-12T20:07:16.1811798Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f677ab38-904a-4499-9434-8cb2d50a0409.sh
2020-04-12T20:07:16.1812208Z 
2020-04-12T20:07:16.1816521Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T20:07:16.1836224Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-12T20:07:16.1839508Z Task         : Get sources
2020-04-12T20:07:16.1839819Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T20:07:16.1840148Z Version      : 1.0.0
2020-04-12T20:07:16.1840353Z Author       : Microsoft
---
2020-04-12T20:07:17.4047224Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T20:07:17.4053106Z ##[command]git config gc.auto 0
2020-04-12T20:07:17.4056906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T20:07:17.4060402Z ##[command]git config --get-all http.proxy
2020-04-12T20:07:17.4068036Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71074/merge:refs/remotes/pull/71074/merge
---
2020-04-12T20:09:27.6947780Z Looks like docker image is the same as before, not uploading
2020-04-12T20:09:35.3998248Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:09:35.4413358Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:09:35.4437986Z == clock drift check ==
2020-04-12T20:09:35.4462672Z   local time: Sun Apr 12 20:09:35 UTC 2020
2020-04-12T20:09:35.5196958Z   network time: Sun, 12 Apr 2020 20:09:35 GMT
2020-04-12T20:09:35.5224922Z Starting sccache server...
2020-04-12T20:09:35.6109293Z configure: processing command line
2020-04-12T20:09:35.6109533Z configure: 
2020-04-12T20:09:35.6110450Z configure: rust.dist-src        := False
---
2020-04-12T20:14:33.9482441Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T20:14:35.3087664Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T20:14:36.8109012Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T20:14:37.6149988Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T20:14:46.0735170Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T20:14:48.0488904Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T20:14:52.1177339Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T20:14:55.9369771Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T20:15:05.1491992Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T20:31:55.0576431Z    Compiling autocfg v0.1.7
2020-04-12T20:31:55.6806675Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:55.6808601Z    --> src/libcore/iter/adapters/mod.rs:166:65
2020-04-12T20:31:55.6809661Z     |
2020-04-12T20:31:55.6811109Z 166 | fn copy_fold<T: Copy, Acc>(mut f: impl FnMut(Acc, T) -> Acc) -> impl FnMut(Acc, &T) -> Acc {
2020-04-12T20:31:55.6813226Z     |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `for<'r> fn(Acc, &'r T) -> _`
2020-04-12T20:31:55.6814884Z 167 |     move |acc, &elt| f(acc, elt)
2020-04-12T20:31:55.6817229Z     |     |
2020-04-12T20:31:55.6817229Z     |     |
2020-04-12T20:31:55.6818410Z     |     found signature of `fn(Acc, &T) -> _`
2020-04-12T20:31:55.6819994Z     |     this returned value is of type `[closure@src/libcore/iter/adapters/mod.rs:167:5: 167:33 f:_]`
2020-04-12T20:31:55.6822373Z     = note: the return type of a function must have a statically known size
2020-04-12T20:31:55.6823004Z 
2020-04-12T20:31:55.6823004Z 
2020-04-12T20:31:55.6825929Z error[E0271]: type mismatch resolving `for<'r> <[closure@src/libcore/iter/adapters/mod.rs:167:5: 167:33 f:_] as ops::function::FnOnce<(Acc, &'r T)>>::Output == Acc`
2020-04-12T20:31:55.6828443Z     |
2020-04-12T20:31:55.6828443Z     |
2020-04-12T20:31:55.6829827Z 166 | fn copy_fold<T: Copy, Acc>(mut f: impl FnMut(Acc, T) -> Acc) -> impl FnMut(Acc, &T) -> Acc {
2020-04-12T20:31:55.6831945Z     |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:55.6834523Z     = note: the return type of a function must have a statically known size
2020-04-12T20:31:55.6835152Z 
2020-04-12T20:31:55.6866403Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:55.6867411Z    --> src/libcore/iter/adapters/mod.rs:170:70
2020-04-12T20:31:55.6867411Z    --> src/libcore/iter/adapters/mod.rs:170:70
2020-04-12T20:31:55.6867935Z     |
2020-04-12T20:31:55.6868748Z 170 | fn copy_try_fold<T: Copy, Acc, R>(mut f: impl FnMut(Acc, T) -> R) -> impl FnMut(Acc, &T) -> R {
2020-04-12T20:31:55.6870055Z     |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `for<'r> fn(Acc, &'r T) -> _`
2020-04-12T20:31:55.6871000Z 171 |     move |acc, &elt| f(acc, elt)
2020-04-12T20:31:55.6872286Z     |     |
2020-04-12T20:31:55.6872286Z     |     |
2020-04-12T20:31:55.6872967Z     |     found signature of `fn(Acc, &T) -> _`
2020-04-12T20:31:55.6873848Z     |     this returned value is of type `[closure@src/libcore/iter/adapters/mod.rs:171:5: 171:33 f:_]`
2020-04-12T20:31:55.6875180Z     = note: the return type of a function must have a statically known size
2020-04-12T20:31:55.6878307Z 
2020-04-12T20:31:55.6878307Z 
2020-04-12T20:31:55.6879640Z error[E0271]: type mismatch resolving `for<'r> <[closure@src/libcore/iter/adapters/mod.rs:171:5: 171:33 f:_] as ops::function::FnOnce<(Acc, &'r T)>>::Output == R`
2020-04-12T20:31:55.6882979Z     |
2020-04-12T20:31:55.6882979Z     |
2020-04-12T20:31:55.6884443Z 170 | fn copy_try_fold<T: Copy, Acc, R>(mut f: impl FnMut(Acc, T) -> R) -> impl FnMut(Acc, &T) -> R {
2020-04-12T20:31:55.6886633Z     |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:55.6889231Z     = note: the return type of a function must have a statically known size
2020-04-12T20:31:55.6889839Z 
2020-04-12T20:31:55.6919719Z error[E0282]: type annotations needed
2020-04-12T20:31:55.6920384Z    --> src/libcore/iter/adapters/mod.rs:313:16
2020-04-12T20:31:55.6920384Z    --> src/libcore/iter/adapters/mod.rs:313:16
2020-04-12T20:31:55.6920889Z     |
2020-04-12T20:31:55.6921507Z 313 |     move |acc, elt| f(acc, elt.clone())
2020-04-12T20:31:55.6922979Z     |
2020-04-12T20:31:55.6924116Z     = note: type must be known at this point
2020-04-12T20:31:55.6924404Z 
2020-04-12T20:31:55.7345825Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:55.7345825Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:55.7346582Z     --> src/libcore/iter/adapters/mod.rs:1620:14
2020-04-12T20:31:55.7347130Z      |
2020-04-12T20:31:55.7347773Z 1620 |           ) -> impl FnMut(&T) -> bool + 'a {
2020-04-12T20:31:55.7348711Z      |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `for<'r> fn(&'r T) -> _`
2020-04-12T20:31:55.7349789Z 1621 |               move |x| {
2020-04-12T20:31:55.7350995Z      |               |
2020-04-12T20:31:55.7350995Z      |               |
2020-04-12T20:31:55.7351760Z      |  _____________found signature of `fn(&T) -> _`
2020-04-12T20:31:55.7352391Z      | |
2020-04-12T20:31:55.7353060Z 1622 | |                 if *flag || !pred(x) {
2020-04-12T20:31:55.7353833Z 1623 | |                     *flag = true;
2020-04-12T20:31:55.7355177Z ...    |
2020-04-12T20:31:55.7355791Z 1627 | |                 }
2020-04-12T20:31:55.7356504Z 1628 | |             }
2020-04-12T20:31:55.7356504Z 1628 | |             }
2020-04-12T20:31:55.7357455Z      | |_____________- this returned value is of type `[closure@src/libcore/iter/adapters/mod.rs:1621:13: 1628:14 flag:_, pred:_]`
2020-04-12T20:31:55.7358778Z      = note: the return type of a function must have a statically known size
2020-04-12T20:31:55.7359101Z 
2020-04-12T20:31:55.7359101Z 
2020-04-12T20:31:55.7359965Z error[E0271]: type mismatch resolving `for<'r> <[closure@src/libcore/iter/adapters/mod.rs:1621:13: 1628:14 flag:_, pred:_] as ops::function::FnOnce<(&'r T,)>>::Output == bool`
2020-04-12T20:31:55.7361291Z      |
2020-04-12T20:31:55.7361291Z      |
2020-04-12T20:31:55.7361855Z 1620 |         ) -> impl FnMut(&T) -> bool + 'a {
2020-04-12T20:31:55.7362724Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:55.7363991Z      = note: the return type of a function must have a statically known size
2020-04-12T20:31:55.7364307Z 
2020-04-12T20:31:55.7633933Z error[E0282]: type annotations needed
2020-04-12T20:31:55.7634690Z    --> src/libcore/iter/adapters/flatten.rs:302:24
2020-04-12T20:31:55.7634690Z    --> src/libcore/iter/adapters/flatten.rs:302:24
2020-04-12T20:31:55.7635223Z     |
2020-04-12T20:31:55.7635811Z 302 |             move |acc, x| {
2020-04-12T20:31:55.7636681Z     |                        ^ consider giving this closure parameter a type
2020-04-12T20:31:55.7642189Z     = note: type must be known at this point
2020-04-12T20:31:55.7642493Z 
2020-04-12T20:31:55.7643022Z error[E0282]: type annotations needed
2020-04-12T20:31:55.7643695Z    --> src/libcore/iter/adapters/flatten.rs:335:24
2020-04-12T20:31:55.7643695Z    --> src/libcore/iter/adapters/flatten.rs:335:24
2020-04-12T20:31:55.7644367Z     |
2020-04-12T20:31:55.7645004Z 335 |             move |acc, iter| iter.fold(acc, &mut *fold)
2020-04-12T20:31:55.7646622Z     |
2020-04-12T20:31:55.7647224Z     = note: type must be known at this point
2020-04-12T20:31:55.7647529Z 
2020-04-12T20:31:55.7658140Z error[E0282]: type annotations needed
2020-04-12T20:31:55.7658140Z error[E0282]: type annotations needed
2020-04-12T20:31:55.7658823Z    --> src/libcore/iter/adapters/flatten.rs:382:24
2020-04-12T20:31:55.7659370Z     |
2020-04-12T20:31:55.7659938Z 382 |             move |acc, x| {
2020-04-12T20:31:55.7660795Z     |                        ^ consider giving this closure parameter a type
2020-04-12T20:31:55.7662037Z     = note: type must be known at this point
2020-04-12T20:31:55.7662327Z 
2020-04-12T20:31:55.7669730Z error[E0282]: type annotations needed
2020-04-12T20:31:55.7670399Z    --> src/libcore/iter/adapters/flatten.rs:415:24
2020-04-12T20:31:55.7670399Z    --> src/libcore/iter/adapters/flatten.rs:415:24
2020-04-12T20:31:55.7670916Z     |
2020-04-12T20:31:55.7671579Z 415 |             move |acc, iter| iter.rfold(acc, &mut *fold)
2020-04-12T20:31:55.7673140Z     |
2020-04-12T20:31:55.7673734Z     = note: type must be known at this point
2020-04-12T20:31:55.7674020Z 
2020-04-12T20:31:55.7877613Z error[E0282]: type annotations needed
---
2020-04-12T20:31:55.7881852Z 
2020-04-12T20:31:55.7890426Z error[E0282]: type annotations needed
2020-04-12T20:31:55.7891108Z     --> src/libcore/iter/traits/iterator.rs:1768:19
2020-04-12T20:31:55.7891662Z      |
2020-04-12T20:31:55.7892272Z 1768 |             move |x| predicate(&**x)
2020-04-12T20:31:55.7893145Z      |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:55.7894367Z      = note: type must be known at this point
2020-04-12T20:31:55.7894658Z 
2020-04-12T20:31:56.2224655Z    Compiling std v0.0.0 (/checkout/src/libstd)
2020-04-12T20:31:56.4992287Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:56.4992287Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:56.4992958Z    --> src/libcore/num/bignum.rs:173:49
2020-04-12T20:31:56.4994033Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.4994033Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.4995079Z 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
2020-04-12T20:31:56.4996040Z 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
2020-04-12T20:31:56.4997392Z ...   |
2020-04-12T20:31:56.4997392Z ...   |
2020-04-12T20:31:56.4998182Z 173 | |                 let zeros = digits.iter().rev().take_while(|&&x| x == 0).count();
2020-04-12T20:31:56.4999306Z     | |                                                 ^^^^^^^^^^ ------------ found signature of `fn(&&_) -> _`
2020-04-12T20:31:56.5000284Z     | |                                                 |
2020-04-12T20:31:56.5001181Z     | |                                                 expected signature of `for<'r> fn(&'r &u32) -> _`
2020-04-12T20:31:56.5002456Z 477 | |     };
2020-04-12T20:31:56.5003103Z 478 | | }
2020-04-12T20:31:56.5003826Z     | |_- in this expansion of `define_bignum!`
2020-04-12T20:31:56.5004327Z ...
2020-04-12T20:31:56.5004327Z ...
2020-04-12T20:31:56.5005205Z 483 |   define_bignum!(Big32x40: type=Digit32, n=40);
2020-04-12T20:31:56.5010499Z 
2020-04-12T20:31:56.5010499Z 
2020-04-12T20:31:56.5016041Z error[E0271]: type mismatch resolving `for<'r> <[closure@src/libcore/num/bignum.rs:173:60: 173:72] as ops::function::FnOnce<(&'r &u32,)>>::Output == bool`
2020-04-12T20:31:56.5016933Z    --> src/libcore/num/bignum.rs:173:49
2020-04-12T20:31:56.5018103Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.5018103Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.5019021Z 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
2020-04-12T20:31:56.5020113Z 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
2020-04-12T20:31:56.5021443Z ...   |
2020-04-12T20:31:56.5021443Z ...   |
2020-04-12T20:31:56.5022220Z 173 | |                 let zeros = digits.iter().rev().take_while(|&&x| x == 0).count();
2020-04-12T20:31:56.5023433Z     | |                                                 ^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:56.5024850Z 477 | |     };
2020-04-12T20:31:56.5025750Z 478 | | }
2020-04-12T20:31:56.5026716Z     | |_- in this expansion of `define_bignum!`
2020-04-12T20:31:56.5027256Z ...
2020-04-12T20:31:56.5027256Z ...
2020-04-12T20:31:56.5027926Z 483 |   define_bignum!(Big32x40: type=Digit32, n=40);
2020-04-12T20:31:56.5066360Z 
2020-04-12T20:31:56.5066360Z 
2020-04-12T20:31:56.5081397Z error[E0599]: no method named `count` found for struct `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u32>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>` in the current scope
2020-04-12T20:31:56.5082309Z     --> src/libcore/num/bignum.rs:173:74
2020-04-12T20:31:56.5083647Z 100  | / macro_rules! define_bignum {
2020-04-12T20:31:56.5083647Z 100  | / macro_rules! define_bignum {
2020-04-12T20:31:56.5084567Z 101  | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
2020-04-12T20:31:56.5085615Z 102  | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
2020-04-12T20:31:56.5087085Z ...    |
2020-04-12T20:31:56.5087085Z ...    |
2020-04-12T20:31:56.5087918Z 173  | |                 let zeros = digits.iter().rev().take_while(|&&x| x == 0).count();
2020-04-12T20:31:56.5089593Z      | |                                                            ------------  ^^^^^ method not found in `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u32>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>`
2020-04-12T20:31:56.5091076Z      | |                                                            |
2020-04-12T20:31:56.5092273Z      | |                                                            doesn't satisfy `<_ as ops::function::FnOnce<(&&u32,)>>::Output = bool`
2020-04-12T20:31:56.5093601Z      | |                                                            doesn't satisfy `_: ops::function::FnMut<(&&u32,)>`
2020-04-12T20:31:56.5095092Z 477  | |     };
2020-04-12T20:31:56.5095808Z 478  | | }
2020-04-12T20:31:56.5096714Z      | |_- in this expansion of `define_bignum!`
2020-04-12T20:31:56.5097258Z ...
2020-04-12T20:31:56.5097258Z ...
2020-04-12T20:31:56.5097949Z 483  |   define_bignum!(Big32x40: type=Digit32, n=40);
2020-04-12T20:31:56.5099616Z      | 
2020-04-12T20:31:56.5100190Z     ::: src/libcore/iter/adapters/mod.rs:1691:1
2020-04-12T20:31:56.5100712Z      |
2020-04-12T20:31:56.5100712Z      |
2020-04-12T20:31:56.5101322Z 1691 |   pub struct TakeWhile<I, P> {
2020-04-12T20:31:56.5102615Z      |   |
2020-04-12T20:31:56.5103274Z      |   method `count` not found for this
2020-04-12T20:31:56.5103274Z      |   method `count` not found for this
2020-04-12T20:31:56.5104075Z      |   doesn't satisfy `_: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5105646Z      = note: the method `count` exists but the following trait bounds were not satisfied:
2020-04-12T20:31:56.5105646Z      = note: the method `count` exists but the following trait bounds were not satisfied:
2020-04-12T20:31:56.5106576Z              `<[closure@src/libcore/num/bignum.rs:173:60: 173:72] as ops::function::FnOnce<(&&u32,)>>::Output = bool`
2020-04-12T20:31:56.5107669Z              which is required by `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u32>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5108654Z              `[closure@src/libcore/num/bignum.rs:173:60: 173:72]: ops::function::FnMut<(&&u32,)>`
2020-04-12T20:31:56.5109694Z              which is required by `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u32>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5110898Z              `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u32>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5112159Z              which is required by `&mut iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u32>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5130666Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:56.5130666Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:56.5131331Z    --> src/libcore/num/bignum.rs:173:49
2020-04-12T20:31:56.5132697Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.5132697Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.5133617Z 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
2020-04-12T20:31:56.5134724Z 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
2020-04-12T20:31:56.5136074Z ...   |
2020-04-12T20:31:56.5136074Z ...   |
2020-04-12T20:31:56.5136834Z 173 | |                 let zeros = digits.iter().rev().take_while(|&&x| x == 0).count();
2020-04-12T20:31:56.5138113Z     | |                                                 ^^^^^^^^^^ ------------ found signature of `fn(&&_) -> _`
2020-04-12T20:31:56.5139128Z     | |                                                 |
2020-04-12T20:31:56.5140052Z     | |                                                 expected signature of `for<'r> fn(&'r &u8) -> _`
2020-04-12T20:31:56.5141323Z 477 | |     };
2020-04-12T20:31:56.5141970Z 478 | | }
2020-04-12T20:31:56.5142676Z     | |_- in this expansion of `define_bignum!`
2020-04-12T20:31:56.5143193Z ...
2020-04-12T20:31:56.5143193Z ...
2020-04-12T20:31:56.5143804Z 488 |       define_bignum!(Big8x3: type=u8, n=3);
2020-04-12T20:31:56.5145022Z 
2020-04-12T20:31:56.5145022Z 
2020-04-12T20:31:56.5146049Z error[E0271]: type mismatch resolving `for<'r> <[closure@src/libcore/num/bignum.rs:173:60: 173:72] as ops::function::FnOnce<(&'r &u8,)>>::Output == bool`
2020-04-12T20:31:56.5146848Z    --> src/libcore/num/bignum.rs:173:49
2020-04-12T20:31:56.5147926Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.5147926Z 100 | / macro_rules! define_bignum {
2020-04-12T20:31:56.5148785Z 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
2020-04-12T20:31:56.5149734Z 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
2020-04-12T20:31:56.5151073Z ...   |
2020-04-12T20:31:56.5151073Z ...   |
2020-04-12T20:31:56.5151829Z 173 | |                 let zeros = digits.iter().rev().take_while(|&&x| x == 0).count();
2020-04-12T20:31:56.5192094Z     | |                                                 ^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:56.5193710Z 477 | |     };
2020-04-12T20:31:56.5194383Z 478 | | }
2020-04-12T20:31:56.5195093Z     | |_- in this expansion of `define_bignum!`
2020-04-12T20:31:56.5195611Z ...
2020-04-12T20:31:56.5195611Z ...
2020-04-12T20:31:56.5196227Z 488 |       define_bignum!(Big8x3: type=u8, n=3);
2020-04-12T20:31:56.5197586Z 
2020-04-12T20:31:56.5197586Z 
2020-04-12T20:31:56.5198583Z error[E0599]: no method named `count` found for struct `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u8>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>` in the current scope
2020-04-12T20:31:56.5199485Z     --> src/libcore/num/bignum.rs:173:74
2020-04-12T20:31:56.5200561Z 100  | / macro_rules! define_bignum {
2020-04-12T20:31:56.5200561Z 100  | / macro_rules! define_bignum {
2020-04-12T20:31:56.5201425Z 101  | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
2020-04-12T20:31:56.5202364Z 102  | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
2020-04-12T20:31:56.5203733Z ...    |
2020-04-12T20:31:56.5203733Z ...    |
2020-04-12T20:31:56.5204978Z 173  | |                 let zeros = digits.iter().rev().take_while(|&&x| x == 0).count();
2020-04-12T20:31:56.5207681Z      | |                                                            ------------  ^^^^^ method not found in `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u8>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>`
2020-04-12T20:31:56.5209117Z      | |                                                            |
2020-04-12T20:31:56.5210295Z      | |                                                            doesn't satisfy `<_ as ops::function::FnOnce<(&&u8,)>>::Output = bool`
2020-04-12T20:31:56.5211632Z      | |                                                            doesn't satisfy `_: ops::function::FnMut<(&&u8,)>`
2020-04-12T20:31:56.5213160Z 477  | |     };
2020-04-12T20:31:56.5213859Z 478  | | }
2020-04-12T20:31:56.5214634Z      | |_- in this expansion of `define_bignum!`
2020-04-12T20:31:56.5215194Z ...
2020-04-12T20:31:56.5215194Z ...
2020-04-12T20:31:56.5215851Z 488  |       define_bignum!(Big8x3: type=u8, n=3);
2020-04-12T20:31:56.5217439Z      | 
2020-04-12T20:31:56.5218011Z     ::: src/libcore/iter/adapters/mod.rs:1691:1
2020-04-12T20:31:56.5218544Z      |
2020-04-12T20:31:56.5218544Z      |
2020-04-12T20:31:56.5219143Z 1691 |   pub struct TakeWhile<I, P> {
2020-04-12T20:31:56.5220542Z      |   |
2020-04-12T20:31:56.5221186Z      |   method `count` not found for this
2020-04-12T20:31:56.5221186Z      |   method `count` not found for this
2020-04-12T20:31:56.5221985Z      |   doesn't satisfy `_: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5223457Z      = note: the method `count` exists but the following trait bounds were not satisfied:
2020-04-12T20:31:56.5223457Z      = note: the method `count` exists but the following trait bounds were not satisfied:
2020-04-12T20:31:56.5224321Z              `<[closure@src/libcore/num/bignum.rs:173:60: 173:72] as ops::function::FnOnce<(&&u8,)>>::Output = bool`
2020-04-12T20:31:56.5225348Z              which is required by `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u8>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5226475Z              `[closure@src/libcore/num/bignum.rs:173:60: 173:72]: ops::function::FnMut<(&&u8,)>`
2020-04-12T20:31:56.5227449Z              which is required by `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u8>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5228544Z              `iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u8>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5229664Z              which is required by `&mut iter::adapters::TakeWhile<iter::adapters::Rev<slice::Iter<'_, u8>>, [closure@src/libcore/num/bignum.rs:173:60: 173:72]>: iter::traits::iterator::Iterator`
2020-04-12T20:31:56.5947413Z    Compiling compiler_builtins v0.1.25
2020-04-12T20:31:56.9229695Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:56.9230802Z    --> src/libcore/cmp.rs:973:5
2020-04-12T20:31:56.9231331Z     |
2020-04-12T20:31:56.9231331Z     |
2020-04-12T20:31:56.9232118Z 948 | pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
2020-04-12T20:31:56.9233290Z     |                     -------------------------- required by this bound in `cmp::min_by`
2020-04-12T20:31:56.9234002Z ...
2020-04-12T20:31:56.9234621Z 973 |     min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
2020-04-12T20:31:56.9235678Z     |     ^^^^^^         -------------------------- found signature of `fn(&T, &T) -> _`
2020-04-12T20:31:56.9236408Z     |     |
2020-04-12T20:31:56.9237083Z     |     expected signature of `for<'r, 's> fn(&'r T, &'s T) -> _`
2020-04-12T20:31:56.9237374Z 
2020-04-12T20:31:56.9238227Z error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/libcore/cmp.rs:973:20: 973:46 f:_] as ops::function::FnOnce<(&'r T, &'s T)>>::Output == cmp::Ordering`
2020-04-12T20:31:56.9257159Z    --> src/libcore/cmp.rs:973:5
2020-04-12T20:31:56.9257683Z     |
2020-04-12T20:31:56.9258412Z 948 | pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
2020-04-12T20:31:56.9259677Z     |                                       -------- required by this bound in `cmp::min_by`
2020-04-12T20:31:56.9260585Z ...
2020-04-12T20:31:56.9261188Z 973 |     min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
2020-04-12T20:31:56.9262068Z     |     ^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:56.9263020Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:56.9263665Z     --> src/libcore/cmp.rs:1039:5
2020-04-12T20:31:56.9264144Z      |
2020-04-12T20:31:56.9264144Z      |
2020-04-12T20:31:56.9264930Z 1014 | pub fn max_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
2020-04-12T20:31:56.9266176Z      |                     -------------------------- required by this bound in `cmp::max_by`
2020-04-12T20:31:56.9267873Z ...
2020-04-12T20:31:56.9268514Z 1039 |     max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
2020-04-12T20:31:56.9269505Z      |     ^^^^^^         -------------------------- found signature of `fn(&T, &T) -> _`
2020-04-12T20:31:56.9270312Z      |     |
2020-04-12T20:31:56.9271025Z      |     expected signature of `for<'r, 's> fn(&'r T, &'s T) -> _`
2020-04-12T20:31:56.9271340Z 
2020-04-12T20:31:56.9272215Z error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/libcore/cmp.rs:1039:20: 1039:46 f:_] as ops::function::FnOnce<(&'r T, &'s T)>>::Output == cmp::Ordering`
2020-04-12T20:31:56.9273089Z     --> src/libcore/cmp.rs:1039:5
2020-04-12T20:31:56.9273568Z      |
2020-04-12T20:31:56.9274361Z 1014 | pub fn max_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
2020-04-12T20:31:56.9275480Z      |                                       -------- required by this bound in `cmp::max_by`
2020-04-12T20:31:56.9276202Z ...
2020-04-12T20:31:56.9276803Z 1039 |     max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
2020-04-12T20:31:56.9277689Z      |     ^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:57.0935689Z error[E0282]: type annotations needed
2020-04-12T20:31:57.0936405Z    --> src/libcore/array/mod.rs:145:39
2020-04-12T20:31:57.0936929Z     |
2020-04-12T20:31:57.0936929Z     |
2020-04-12T20:31:57.0937585Z 145 |         <&Self>::try_from(slice).map(|r| *r)
2020-04-12T20:31:57.0938553Z     |                                       ^ consider giving this closure parameter a type
2020-04-12T20:31:57.0939795Z     = note: type must be known at this point
2020-04-12T20:31:57.0940101Z 
2020-04-12T20:31:57.3067875Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:57.3068598Z     --> src/libcore/iter/adapters/mod.rs:2420:14
2020-04-12T20:31:57.3068598Z     --> src/libcore/iter/adapters/mod.rs:2420:14
2020-04-12T20:31:57.3069303Z      |
2020-04-12T20:31:57.3070032Z 2420 |         self.find(|_| true)
2020-04-12T20:31:57.3070947Z      |              ^^^^ -------- found signature of `fn(_) -> _`
2020-04-12T20:31:57.3071806Z      |              |
2020-04-12T20:31:57.3072540Z      |              expected signature of `for<'r> fn(&'r T) -> _`
2020-04-12T20:31:57.3072866Z 
2020-04-12T20:31:57.3073693Z error[E0271]: type mismatch resolving `for<'r> <[closure@src/libcore/iter/adapters/mod.rs:2420:19: 2420:27] as ops::function::FnOnce<(&'r T,)>>::Output == bool`
2020-04-12T20:31:57.3075094Z      |
2020-04-12T20:31:57.3075094Z      |
2020-04-12T20:31:57.3075657Z 2420 |         self.find(|_| true)
2020-04-12T20:31:57.3076512Z      |              ^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:57.4643131Z error[E0282]: type annotations needed
2020-04-12T20:31:57.4644244Z    --> src/libcore/iter/traits/accum.rs:144:38
2020-04-12T20:31:57.4644788Z     |
2020-04-12T20:31:57.4644788Z     |
2020-04-12T20:31:57.4645451Z 144 |         iter::process_results(iter, |i| i.sum())
2020-04-12T20:31:57.4646412Z     |                                      ^ consider giving this closure parameter a type
2020-04-12T20:31:57.4647657Z     = note: type must be known at this point
2020-04-12T20:31:57.4650640Z 
2020-04-12T20:31:57.4651318Z error[E0282]: type annotations needed
2020-04-12T20:31:57.4651905Z    --> src/libcore/iter/traits/accum.rs:160:38
2020-04-12T20:31:57.4651905Z    --> src/libcore/iter/traits/accum.rs:160:38
2020-04-12T20:31:57.4652400Z     |
2020-04-12T20:31:57.4653016Z 160 |         iter::process_results(iter, |i| i.product())
2020-04-12T20:31:57.4653892Z     |                                      ^ consider giving this closure parameter a type
2020-04-12T20:31:57.4655051Z     = note: type must be known at this point
2020-04-12T20:31:57.4655320Z 
2020-04-12T20:31:57.4655763Z error[E0282]: type annotations needed
2020-04-12T20:31:57.4656340Z    --> src/libcore/iter/traits/accum.rs:187:19
2020-04-12T20:31:57.4656340Z    --> src/libcore/iter/traits/accum.rs:187:19
2020-04-12T20:31:57.4656803Z     |
2020-04-12T20:31:57.4657466Z 187 |         iter.map(|x| x.ok_or(())).sum::<Result<_, _>>().ok()
2020-04-12T20:31:57.4658331Z     |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:57.4661990Z     = note: type must be known at this point
2020-04-12T20:31:57.4662278Z 
2020-04-12T20:31:57.4662859Z error[E0282]: type annotations needed
2020-04-12T20:31:57.4663628Z    --> src/libcore/iter/traits/accum.rs:203:19
2020-04-12T20:31:57.4663628Z    --> src/libcore/iter/traits/accum.rs:203:19
2020-04-12T20:31:57.4664093Z     |
2020-04-12T20:31:57.4664748Z 203 |         iter.map(|x| x.ok_or(())).product::<Result<_, _>>().ok()
2020-04-12T20:31:57.4665808Z     |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:57.4666942Z     = note: type must be known at this point
2020-04-12T20:31:57.4667221Z 
2020-04-12T20:31:57.5027262Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5027930Z     --> src/libcore/iter/traits/iterator.rs:2856:29
2020-04-12T20:31:57.5027930Z     --> src/libcore/iter/traits/iterator.rs:2856:29
2020-04-12T20:31:57.5028641Z      |
2020-04-12T20:31:57.5029282Z 2856 |         self.cmp_by(other, |x, y| x.cmp(&y))
2020-04-12T20:31:57.5030164Z      |                             ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5031423Z      = note: type must be known at this point
2020-04-12T20:31:57.5031713Z 
2020-04-12T20:31:57.5050975Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5051665Z     --> src/libcore/iter/traits/iterator.rs:2932:37
2020-04-12T20:31:57.5051665Z     --> src/libcore/iter/traits/iterator.rs:2932:37
2020-04-12T20:31:57.5052188Z      |
2020-04-12T20:31:57.5052862Z 2932 |         self.partial_cmp_by(other, |x, y| x.partial_cmp(&y))
2020-04-12T20:31:57.5053805Z      |                                     ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5055072Z      = note: type must be known at this point
2020-04-12T20:31:57.5055361Z 
2020-04-12T20:31:57.5304719Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5305378Z     --> src/libcore/option.rs:1028:19
2020-04-12T20:31:57.5305378Z     --> src/libcore/option.rs:1028:19
2020-04-12T20:31:57.5308788Z      |
2020-04-12T20:31:57.5309662Z 1028 |         self.map(|t| t.clone())
2020-04-12T20:31:57.5310679Z      |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5311927Z      = note: type must be known at this point
2020-04-12T20:31:57.5312218Z 
2020-04-12T20:31:57.5312740Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5313346Z     --> src/libcore/option.rs:1047:19
2020-04-12T20:31:57.5313346Z     --> src/libcore/option.rs:1047:19
2020-04-12T20:31:57.5313853Z      |
2020-04-12T20:31:57.5314427Z 1047 |         self.map(|t| t.clone())
2020-04-12T20:31:57.5315258Z      |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5316487Z      = note: type must be known at this point
2020-04-12T20:31:57.5316773Z 
2020-04-12T20:31:57.5323090Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5323718Z     --> src/libcore/option.rs:1197:28
2020-04-12T20:31:57.5323718Z     --> src/libcore/option.rs:1197:28
2020-04-12T20:31:57.5324211Z      |
2020-04-12T20:31:57.5324838Z 1197 |         self.as_ref().map(|t| t.deref())
2020-04-12T20:31:57.5325844Z      |                            ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5327109Z      = note: type must be known at this point
2020-04-12T20:31:57.5330027Z 
2020-04-12T20:31:57.5330685Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5331313Z     --> src/libcore/option.rs:1218:28
2020-04-12T20:31:57.5331313Z     --> src/libcore/option.rs:1218:28
2020-04-12T20:31:57.5331814Z      |
2020-04-12T20:31:57.5332442Z 1218 |         self.as_mut().map(|t| t.deref_mut())
2020-04-12T20:31:57.5333388Z      |                            ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5334546Z      = note: type must be known at this point
2020-04-12T20:31:57.5334810Z 
2020-04-12T20:31:57.5434009Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5434636Z     --> src/libcore/option.rs:1630:31
2020-04-12T20:31:57.5434636Z     --> src/libcore/option.rs:1630:31
2020-04-12T20:31:57.5435097Z      |
2020-04-12T20:31:57.5435798Z 1630 |         iter.into_iter().map(|x| x.ok_or(())).collect::<Result<_, _>>().ok()
2020-04-12T20:31:57.5436750Z      |                               ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5437894Z      = note: type must be known at this point
2020-04-12T20:31:57.5438180Z 
2020-04-12T20:31:57.5826586Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5831024Z    --> src/libcore/result.rs:914:19
2020-04-12T20:31:57.5831024Z    --> src/libcore/result.rs:914:19
2020-04-12T20:31:57.5831582Z     |
2020-04-12T20:31:57.5832167Z 914 |         self.map(|t| t.clone())
2020-04-12T20:31:57.5833003Z     |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5834260Z     = note: type must be known at this point
2020-04-12T20:31:57.5834547Z 
2020-04-12T20:31:57.5835087Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5835682Z    --> src/libcore/result.rs:934:19
2020-04-12T20:31:57.5835682Z    --> src/libcore/result.rs:934:19
2020-04-12T20:31:57.5836172Z     |
2020-04-12T20:31:57.5836758Z 934 |         self.map(|t| t.clone())
2020-04-12T20:31:57.5837589Z     |                   ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5838803Z     = note: type must be known at this point
2020-04-12T20:31:57.5839235Z 
2020-04-12T20:31:57.5851975Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5852597Z     --> src/libcore/result.rs:1155:28
2020-04-12T20:31:57.5852597Z     --> src/libcore/result.rs:1155:28
2020-04-12T20:31:57.5853056Z      |
2020-04-12T20:31:57.5853617Z 1155 |         self.as_ref().map(|t| t.deref())
2020-04-12T20:31:57.5854596Z      |                            ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5855753Z      = note: type must be known at this point
2020-04-12T20:31:57.5858632Z 
2020-04-12T20:31:57.5859253Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5859818Z     --> src/libcore/result.rs:1166:32
2020-04-12T20:31:57.5859818Z     --> src/libcore/result.rs:1166:32
2020-04-12T20:31:57.5860309Z      |
2020-04-12T20:31:57.5860876Z 1166 |         self.as_ref().map_err(|e| e.deref())
2020-04-12T20:31:57.5861697Z      |                                ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5862859Z      = note: type must be known at this point
2020-04-12T20:31:57.5863126Z 
2020-04-12T20:31:57.5867091Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5867665Z     --> src/libcore/result.rs:1177:28
2020-04-12T20:31:57.5867665Z     --> src/libcore/result.rs:1177:28
2020-04-12T20:31:57.5868123Z      |
2020-04-12T20:31:57.5868715Z 1177 |         self.as_mut().map(|t| t.deref_mut())
2020-04-12T20:31:57.5869541Z      |                            ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5870693Z      = note: type must be known at this point
2020-04-12T20:31:57.5870963Z 
2020-04-12T20:31:57.5875300Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5875943Z     --> src/libcore/result.rs:1188:32
2020-04-12T20:31:57.5875943Z     --> src/libcore/result.rs:1188:32
2020-04-12T20:31:57.5876437Z      |
2020-04-12T20:31:57.5877058Z 1188 |         self.as_mut().map_err(|e| e.deref_mut())
2020-04-12T20:31:57.5877974Z      |                                ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5879231Z      = note: type must be known at this point
2020-04-12T20:31:57.5879520Z 
2020-04-12T20:31:57.5957972Z error[E0282]: type annotations needed
2020-04-12T20:31:57.5958586Z     --> src/libcore/result.rs:1537:50
2020-04-12T20:31:57.5958586Z     --> src/libcore/result.rs:1537:50
2020-04-12T20:31:57.5959071Z      |
2020-04-12T20:31:57.5959728Z 1537 |         iter::process_results(iter.into_iter(), |i| i.collect())
2020-04-12T20:31:57.5960678Z      |                                                  ^ consider giving this closure parameter a type
2020-04-12T20:31:57.5961992Z      = note: type must be known at this point
2020-04-12T20:31:57.5962271Z 
2020-04-12T20:31:57.6508556Z error[E0282]: type annotations needed
2020-04-12T20:31:57.6509177Z    --> src/libcore/fmt/mod.rs:356:60
2020-04-12T20:31:57.6509177Z    --> src/libcore/fmt/mod.rs:356:60
2020-04-12T20:31:57.6509669Z     |
2020-04-12T20:31:57.6510533Z 356 |         let pieces_length: usize = self.pieces.iter().map(|x| x.len()).sum();
2020-04-12T20:31:57.6511530Z     |                                                            ^ consider giving this closure parameter a type
2020-04-12T20:31:57.6512732Z     = note: type must be known at this point
2020-04-12T20:31:57.6513022Z 
2020-04-12T20:31:57.6513022Z 
2020-04-12T20:31:57.7173368Z error[E0271]: type mismatch resolving `<[closure@src/libcore/fmt/builders.rs:24:22: 27:10 state:_, slot:_] as ops::function::FnOnce<(&mut dyn fmt::Write,)>>::Output == &mut dyn fmt::Write`
2020-04-12T20:31:57.7174375Z   --> src/libcore/fmt/builders.rs:24:13
2020-04-12T20:31:57.7174914Z    |
2020-04-12T20:31:57.7175511Z 24 |         fmt.wrap_buf(move |buf| {
2020-04-12T20:31:57.7176545Z    |             ^^^^^^^^ expected struct `fmt::builders::PadAdapter`, found trait object `dyn fmt::Write`
2020-04-12T20:31:57.7177302Z    |
2020-04-12T20:31:57.7178071Z    = note: expected mutable reference `&mut fmt::builders::PadAdapter<'buf, 'state>`
2020-04-12T20:31:57.7178921Z               found mutable reference `&mut dyn fmt::Write`
2020-04-12T20:31:57.8840393Z error[E0282]: type annotations needed
2020-04-12T20:31:57.8841245Z     --> src/libcore/slice/mod.rs:1503:32
2020-04-12T20:31:57.8841750Z      |
2020-04-12T20:31:57.8841750Z      |
2020-04-12T20:31:57.8842426Z 1503 |         self.binary_search_by(|p| p.cmp(x))
2020-04-12T20:31:57.8843323Z      |                                ^ consider giving this closure parameter a type
2020-04-12T20:31:57.8844582Z      = note: type must be known at this point
2020-04-12T20:31:57.8844872Z 
2020-04-12T20:31:57.8887220Z error[E0282]: type annotations needed
2020-04-12T20:31:57.8887885Z     --> src/libcore/slice/mod.rs:1638:32
2020-04-12T20:31:57.8887885Z     --> src/libcore/slice/mod.rs:1638:32
2020-04-12T20:31:57.8888382Z      |
2020-04-12T20:31:57.8889066Z 1638 |         sort::quicksort(self, |a, b| a.lt(b));
2020-04-12T20:31:57.8890030Z      |                                ^ consider giving this closure parameter a type
2020-04-12T20:31:57.8891289Z      = note: type must be known at this point
2020-04-12T20:31:57.8891578Z 
2020-04-12T20:31:57.8908553Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:57.8909245Z     --> src/libcore/slice/mod.rs:1693:9
2020-04-12T20:31:57.8909245Z     --> src/libcore/slice/mod.rs:1693:9
2020-04-12T20:31:57.8909950Z      |
2020-04-12T20:31:57.8910720Z 1693 |         sort::quicksort(self, |a, b| compare(a, b) == Ordering::Less);
2020-04-12T20:31:57.8911959Z      |         ^^^^^^^^^^^^^^^       -------------------------------------- found signature of `fn(&T, &T) -> _`
2020-04-12T20:31:57.8912816Z      |         |
2020-04-12T20:31:57.8913559Z      |         expected signature of `for<'r, 's> fn(&'r T, &'s T) -> _`
2020-04-12T20:31:57.8914704Z     ::: src/libcore/slice/sort.rs:688:8
2020-04-12T20:31:57.8915209Z      |
2020-04-12T20:31:57.8915209Z      |
2020-04-12T20:31:57.8915890Z 688  | pub fn quicksort<T, F>(v: &mut [T], mut is_less: F)
2020-04-12T20:31:57.8917218Z 689  | where
2020-04-12T20:31:57.8917218Z 689  | where
2020-04-12T20:31:57.8917873Z 690  |     F: FnMut(&T, &T) -> bool,
2020-04-12T20:31:57.8918887Z      |        --------------------- required by this bound in `slice::sort::quicksort`
2020-04-12T20:31:57.8919359Z 
2020-04-12T20:31:57.8933421Z error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/libcore/slice/mod.rs:1693:31: 1693:69 compare:_] as ops::function::FnOnce<(&'r T, &'s T)>>::Output == bool`
2020-04-12T20:31:57.8934922Z      |
2020-04-12T20:31:57.8934922Z      |
2020-04-12T20:31:57.8935629Z 1693 |         sort::quicksort(self, |a, b| compare(a, b) == Ordering::Less);
2020-04-12T20:31:57.8936562Z      |         ^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:57.8937938Z     ::: src/libcore/slice/sort.rs:688:8
2020-04-12T20:31:57.8938526Z      |
2020-04-12T20:31:57.8938526Z      |
2020-04-12T20:31:57.8939383Z 688  | pub fn quicksort<T, F>(v: &mut [T], mut is_less: F)
2020-04-12T20:31:57.8940962Z 689  | where
2020-04-12T20:31:57.8940962Z 689  | where
2020-04-12T20:31:57.8941645Z 690  |     F: FnMut(&T, &T) -> bool,
2020-04-12T20:31:57.8942627Z      |                         ---- required by this bound in `slice::sort::quicksort`
2020-04-12T20:31:57.8962208Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:57.8962874Z     --> src/libcore/slice/mod.rs:1732:9
2020-04-12T20:31:57.8963372Z      |
2020-04-12T20:31:57.8963372Z      |
2020-04-12T20:31:57.8964061Z 1732 |         sort::quicksort(self, |a, b| f(a).lt(&f(b)));
2020-04-12T20:31:57.8965112Z      |         ^^^^^^^^^^^^^^^       --------------------- found signature of `fn(&T, &T) -> _`
2020-04-12T20:31:57.8966116Z      |         |
2020-04-12T20:31:57.8966843Z      |         expected signature of `for<'r, 's> fn(&'r T, &'s T) -> _`
2020-04-12T20:31:57.8968064Z     ::: src/libcore/slice/sort.rs:688:8
2020-04-12T20:31:57.8968583Z      |
2020-04-12T20:31:57.8968583Z      |
2020-04-12T20:31:57.8969262Z 688  | pub fn quicksort<T, F>(v: &mut [T], mut is_less: F)
2020-04-12T20:31:57.8970588Z 689  | where
2020-04-12T20:31:57.8970588Z 689  | where
2020-04-12T20:31:57.8971267Z 690  |     F: FnMut(&T, &T) -> bool,
2020-04-12T20:31:57.8972261Z      |        --------------------- required by this bound in `slice::sort::quicksort`
2020-04-12T20:31:57.8972727Z 
2020-04-12T20:31:57.8973591Z error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/libcore/slice/mod.rs:1732:31: 1732:52 f:_] as ops::function::FnOnce<(&'r T, &'s T)>>::Output == bool`
2020-04-12T20:31:57.8974948Z      |
2020-04-12T20:31:57.8974948Z      |
2020-04-12T20:31:57.8975633Z 1732 |         sort::quicksort(self, |a, b| f(a).lt(&f(b)));
2020-04-12T20:31:57.8976581Z      |         ^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:57.8977794Z     ::: src/libcore/slice/sort.rs:688:8
2020-04-12T20:31:57.8978296Z      |
2020-04-12T20:31:57.8978296Z      |
2020-04-12T20:31:57.8978988Z 688  | pub fn quicksort<T, F>(v: &mut [T], mut is_less: F)
2020-04-12T20:31:57.8980298Z 689  | where
2020-04-12T20:31:57.8980298Z 689  | where
2020-04-12T20:31:57.8980976Z 690  |     F: FnMut(&T, &T) -> bool,
2020-04-12T20:31:57.8981952Z      |                         ---- required by this bound in `slice::sort::quicksort`
2020-04-12T20:31:57.9028933Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:57.9029622Z     --> src/libcore/slice/mod.rs:1918:14
2020-04-12T20:31:57.9030120Z      |
2020-04-12T20:31:57.9030120Z      |
2020-04-12T20:31:57.9030755Z 1918 |         self.partition_dedup_by(|a, b| a == b)
2020-04-12T20:31:57.9031746Z      |              ^^^^^^^^^^^^^^^^^^ ------------- found signature of `fn(_, _) -> _`
2020-04-12T20:31:57.9032575Z      |              |
2020-04-12T20:31:57.9033348Z      |              expected signature of `for<'r, 's> fn(&'r mut T, &'s mut T) -> _`
2020-04-12T20:31:57.9033694Z 
2020-04-12T20:31:57.9034580Z error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/libcore/slice/mod.rs:1918:33: 1918:46] as ops::function::FnOnce<(&'r mut T, &'s mut T)>>::Output == bool`
2020-04-12T20:31:57.9036224Z      |
2020-04-12T20:31:57.9036224Z      |
2020-04-12T20:31:57.9036797Z 1918 |         self.partition_dedup_by(|a, b| a == b)
2020-04-12T20:31:57.9037721Z      |              ^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:57.9076144Z error[E0631]: type mismatch in closure arguments
2020-04-12T20:31:57.9076830Z     --> src/libcore/slice/mod.rs:2064:14
2020-04-12T20:31:57.9077333Z      |
2020-04-12T20:31:57.9077333Z      |
2020-04-12T20:31:57.9077979Z 2064 |         self.partition_dedup_by(|a, b| key(a) == key(b))
2020-04-12T20:31:57.9079066Z      |              ^^^^^^^^^^^^^^^^^^ ----------------------- found signature of `fn(&mut T, &mut T) -> _`
2020-04-12T20:31:57.9079907Z      |              |
2020-04-12T20:31:57.9080684Z      |              expected signature of `for<'r, 's> fn(&'r mut T, &'s mut T) -> _`
2020-04-12T20:31:57.9081044Z 
2020-04-12T20:31:57.9081921Z error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/libcore/slice/mod.rs:2064:33: 2064:56 key:_] as ops::function::FnOnce<(&'r mut T, &'s mut T)>>::Output == bool`
2020-04-12T20:31:57.9083325Z      |
2020-04-12T20:31:57.9083325Z      |
2020-04-12T20:31:57.9084071Z 2064 |         self.partition_dedup_by(|a, b| key(a) == key(b))
2020-04-12T20:31:57.9084940Z      |              ^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-04-12T20:31:57.9229583Z error[E0282]: type annotations needed
2020-04-12T20:31:57.9230190Z     --> src/libcore/slice/mod.rs:2623:28
2020-04-12T20:31:57.9230678Z      |
2020-04-12T20:31:57.9230678Z      |
2020-04-12T20:31:57.9231258Z 2623 |         self.is_sorted_by(|a, b| a.partial_cmp(b))
2020-04-12T20:31:57.9232106Z      |                            ^ consider giving this closure parameter a type
2020-04-12T20:31:57.9233252Z      = note: type must be known at this point
2020-04-12T20:31:57.9233520Z 
2020-04-12T20:31:57.9245926Z error[E0282]: type annotations needed
2020-04-12T20:31:57.9246535Z     --> src/libcore/slice/mod.rs:2638:35
2020-04-12T20:31:57.9246535Z     --> src/libcore/slice/mod.rs:2638:35
2020-04-12T20:31:57.9247000Z      |
2020-04-12T20:31:57.9247615Z 2638 |         self.iter().is_sorted_by(|a, b| compare(*a, *b))
---
2020-04-12T20:32:04.7132360Z expected success, got: exit code: 101
2020-04-12T20:32:04.7143730Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-12T20:32:04.7144120Z Build completed unsuccessfully in 0:20:51
2020-04-12T20:32:04.7200375Z == clock drift check ==
2020-04-12T20:32:04.7220189Z   local time: Sun Apr 12 20:32:04 UTC 2020
2020-04-12T20:32:04.8383821Z   network time: Sun, 12 Apr 2020 20:32:04 GMT
2020-04-12T20:32:06.5362725Z 
2020-04-12T20:32:06.5362725Z 
2020-04-12T20:32:06.5427746Z ##[error]Bash exited with code '1'.
2020-04-12T20:32:06.5440953Z ##[section]Finishing: Run build
2020-04-12T20:32:06.5502042Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-12T20:32:06.5507929Z Task         : Get sources
2020-04-12T20:32:06.5508243Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T20:32:06.5508554Z Version      : 1.0.0
2020-04-12T20:32:06.5508764Z Author       : Microsoft
2020-04-12T20:32:06.5508764Z Author       : Microsoft
2020-04-12T20:32:06.5509085Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T20:32:06.5509650Z ==============================================================================
2020-04-12T20:32:06.8619882Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T20:32:06.8684325Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-12T20:32:06.8764260Z Cleaning up task key
2020-04-12T20:32:06.8765443Z Start cleaning up orphan processes.
2020-04-12T20:32:06.8932935Z Terminate orphan process: pid (3481) (python)
2020-04-12T20:32:06.9243997Z ##[section]Finishing: Finalize Job
