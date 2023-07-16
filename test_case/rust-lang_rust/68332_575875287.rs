plain
2020-01-18T07:30:04.9189909Z ========================== Starting Command Output ===========================
2020-01-18T07:30:04.9191601Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ea3640b7-15ae-4a71-a402-3477eeba57e1.sh
2020-01-18T07:30:04.9191634Z 
2020-01-18T07:30:04.9193754Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T07:30:04.9199876Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:30:04.9201470Z Task         : Get sources
2020-01-18T07:30:04.9201503Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:30:04.9201535Z Version      : 1.0.0
2020-01-18T07:30:04.9201588Z Author       : Microsoft
---
2020-01-18T07:30:05.9175688Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T07:30:05.9265778Z ##[command]git config gc.auto 0
2020-01-18T07:30:05.9339315Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T07:30:05.9372158Z ##[command]git config --get-all http.proxy
2020-01-18T07:30:05.9550663Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T07:34:32.8157314Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T07:34:34.4408865Z error: expected unsuffixed literal or identifier, found `fn_not_impl`
2020-01-18T07:34:34.4409189Z    --> src/libcore/ops/function.rs:378:41
2020-01-18T07:34:34.4409585Z     |
2020-01-18T07:34:34.4409855Z 378 | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:34:34.4410150Z 
2020-01-18T07:34:34.4425986Z error: expected unsuffixed literal or identifier, found `fn_neg_impl`
2020-01-18T07:34:34.4426476Z    --> src/libcore/ops/function.rs:379:41
2020-01-18T07:34:34.4426809Z     |
2020-01-18T07:34:34.4426809Z     |
2020-01-18T07:34:34.4427136Z 379 | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:34:34.4427496Z 
2020-01-18T07:34:37.6310238Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:34:37.6317955Z    --> src/libcore/ops/function.rs:304:18
2020-01-18T07:34:37.6318574Z     |
2020-01-18T07:34:37.6318574Z     |
2020-01-18T07:34:37.6319201Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:34:37.6320203Z     |   _-
2020-01-18T07:34:37.6320773Z     |  |_|
2020-01-18T07:34:37.6321309Z     |  |
2020-01-18T07:34:37.6322771Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:34:37.6323568Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:34:37.6324277Z     |  |_________-
2020-01-18T07:34:37.6324981Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:34:37.6325840Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:34:37.6326388Z ...    |
2020-01-18T07:34:37.6327041Z 304 |  |             impl<A, F: ?Sized> $imp for F
2020-01-18T07:34:37.6327684Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:34:37.6328798Z 376 |  |     };
2020-01-18T07:34:37.6329379Z 377 |  | }
2020-01-18T07:34:37.6330104Z     |  | -
2020-01-18T07:34:37.6330610Z     |  |_|
2020-01-18T07:34:37.6330610Z     |  |_|
2020-01-18T07:34:37.6331213Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6332146Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6332911Z 378 |  | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:34:37.6333941Z 
2020-01-18T07:34:37.6334645Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:34:37.6335223Z    --> src/libcore/ops/function.rs:314:18
2020-01-18T07:34:37.6335893Z     |
2020-01-18T07:34:37.6335893Z     |
2020-01-18T07:34:37.6336447Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:34:37.6336977Z     |   _-
2020-01-18T07:34:37.6337481Z     |  |_|
2020-01-18T07:34:37.6337986Z     |  |
2020-01-18T07:34:37.6338612Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:34:37.6339409Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:34:37.6340015Z     |  |_________-
2020-01-18T07:34:37.6340712Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:34:37.6341432Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:34:37.6342420Z ...    |
2020-01-18T07:34:37.6343140Z 314 |  |             impl<A, F: ?Sized> $imp for $z<F>
2020-01-18T07:34:37.6344713Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:34:37.6345951Z 376 |  |     };
2020-01-18T07:34:37.6346576Z 377 |  | }
2020-01-18T07:34:37.6347056Z     |  | -
2020-01-18T07:34:37.6347453Z     |  |_|
2020-01-18T07:34:37.6347453Z     |  |_|
2020-01-18T07:34:37.6387298Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6387898Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6388470Z 378 |  | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:34:37.6389255Z 
2020-01-18T07:34:37.6390103Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:34:37.6390717Z    --> src/libcore/ops/function.rs:304:18
2020-01-18T07:34:37.6391141Z     |
2020-01-18T07:34:37.6391141Z     |
2020-01-18T07:34:37.6391600Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:34:37.6392448Z     |   _-
2020-01-18T07:34:37.6392901Z     |  |_|
2020-01-18T07:34:37.6393318Z     |  |
2020-01-18T07:34:37.6393881Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:34:37.6394428Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:34:37.6395265Z     |  |_________-
2020-01-18T07:34:37.6395730Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:34:37.6396633Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:34:37.6397037Z ...    |
2020-01-18T07:34:37.6397613Z 304 |  |             impl<A, F: ?Sized> $imp for F
2020-01-18T07:34:37.6398176Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:34:37.6399015Z 376 |  |     };
2020-01-18T07:34:37.6399606Z 377 |  | }
2020-01-18T07:34:37.6400068Z     |  | -
2020-01-18T07:34:37.6400462Z     |  |_|
2020-01-18T07:34:37.6400462Z     |  |_|
2020-01-18T07:34:37.6400928Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6401395Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6402650Z 378 |    gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:34:37.6403262Z 379 |  | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:34:37.6404079Z 
2020-01-18T07:34:37.6404539Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:34:37.6404988Z    --> src/libcore/ops/function.rs:314:18
2020-01-18T07:34:37.6405405Z     |
2020-01-18T07:34:37.6405405Z     |
2020-01-18T07:34:37.6405863Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:34:37.6406299Z     |   _-
2020-01-18T07:34:37.6406714Z     |  |_|
2020-01-18T07:34:37.6407195Z     |  |
2020-01-18T07:34:37.6408095Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:34:37.6408652Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:34:37.6409106Z     |  |_________-
2020-01-18T07:34:37.6409599Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:34:37.6410163Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:34:37.6410591Z ...    |
2020-01-18T07:34:37.6411073Z 314 |  |             impl<A, F: ?Sized> $imp for $z<F>
2020-01-18T07:34:37.6412166Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:34:37.6413374Z 376 |  |     };
2020-01-18T07:34:37.6413877Z 377 |  | }
2020-01-18T07:34:37.6416274Z     |  | -
2020-01-18T07:34:37.6416533Z     |  |_|
2020-01-18T07:34:37.6416533Z     |  |_|
2020-01-18T07:34:37.6416858Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6417159Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:34:37.6417473Z 378 |    gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:34:37.6417829Z 379 |  | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:34:37.6418278Z 
2020-01-18T07:34:37.6418515Z error: aborting due to 6 previous errors
2020-01-18T07:34:37.6418549Z 
2020-01-18T07:34:37.6418802Z For more information about this error, try `rustc --explain E0207`.
---
2020-01-18T07:34:39.6048260Z   local time: Sat Jan 18 07:34:39 UTC 2020
2020-01-18T07:34:40.1384371Z   network time: Sat, 18 Jan 2020 07:34:40 GMT
2020-01-18T07:34:40.1390381Z == end clock drift check ==
2020-01-18T07:34:48.1211190Z 
2020-01-18T07:34:48.1357484Z ##[error]Bash exited with code '1'.
2020-01-18T07:34:48.1370464Z ##[section]Finishing: Run build
2020-01-18T07:34:48.1386888Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:34:48.1388971Z Task         : Get sources
2020-01-18T07:34:48.1389011Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:34:48.1389067Z Version      : 1.0.0
2020-01-18T07:34:48.1389101Z Author       : Microsoft
2020-01-18T07:34:48.1389101Z Author       : Microsoft
2020-01-18T07:34:48.1389141Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T07:34:48.1389331Z ==============================================================================
2020-01-18T07:34:48.5728649Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T07:34:48.5766591Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:34:48.5881833Z Cleaning up task key
2020-01-18T07:34:48.5883197Z Start cleaning up orphan processes.
2020-01-18T07:34:48.6001455Z Terminate orphan process: pid (3909) (python)
2020-01-18T07:34:48.6199169Z ##[section]Finishing: Finalize Job
