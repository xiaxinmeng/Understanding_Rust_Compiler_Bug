plain
2019-10-11T01:43:19.0836625Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T01:43:19.0923997Z ##[command]git config gc.auto 0
2019-10-11T01:43:19.1011319Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T01:43:19.1078670Z ##[command]git config --get-all http.proxy
2019-10-11T01:43:19.1214486Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65294/merge:refs/remotes/pull/65294/merge
---
2019-10-11T02:15:09.5374260Z    Compiling jobserver v0.1.16
2019-10-11T02:15:11.9251952Z    Compiling env_logger v0.7.0
2019-10-11T02:15:12.5820649Z    Compiling rustc-hash v1.0.1
2019-10-11T02:15:12.6895384Z    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-10-11T02:15:12.8708523Z error[E0518]: attribute should be applied to function or closure
2019-10-11T02:15:12.8708962Z    --> <::bitflags::__impl_bitflags macros>:73:22
2019-10-11T02:15:12.8709318Z     |
2019-10-11T02:15:12.8711001Z 1   |   /   ($ BitFlags : ident : $ T : ty
2019-10-11T02:15:12.8711767Z 3   |   |        $
2019-10-11T02:15:12.8711767Z 3   |   |        $
2019-10-11T02:15:12.8712190Z 4   |   |        ($ (# [$ attr : ident $ ($ args : tt) *]) * $ Flag : ident = $ value :
2019-10-11T02:15:12.8712510Z ...     |
2019-10-11T02:15:12.8712925Z 73  |   |                   { $ (# [inline] const $ Flag : $ T = 0 ;) + } impl __BitFlags
2019-10-11T02:15:12.8713675Z     |   |                        ^^^^^^^^^^ ------------------------ not a function or closure
2019-10-11T02:15:12.8714041Z ...     |
2019-10-11T02:15:12.8714456Z 248 |   |   } ; ($ (# [$ filtered : meta]) * const $ ($ item : tt) *) =>
2019-10-11T02:15:12.8714856Z 249 |   |   { $ (# [$ filtered]) * const $ ($ item) * } ;
2019-10-11T02:15:12.8715668Z     |   |_______________________________________________- in this expansion of `__impl_bitflags!` (#3)
2019-10-11T02:15:12.8715986Z     | 
2019-10-11T02:15:12.8716573Z    ::: <::bitflags::__bitflags macros>:1:1
2019-10-11T02:15:12.8716882Z     |
2019-10-11T02:15:12.8717323Z 1   |     / ($ (# [$ outer : meta]) * ($ ($ vis : tt) *) $ BitFlags : ident : $ T : ty
2019-10-11T02:15:12.8717724Z 2   |     |  {
2019-10-11T02:15:12.8718091Z 3   |     |      $
2019-10-11T02:15:12.8718539Z 4   |     |      ($ (# [$ inner : ident $ ($ args : tt) *]) * $ Flag : ident = $ value :
2019-10-11T02:15:12.8718871Z ...       |
2019-10-11T02:15:12.8719303Z 10  |     |     struct $ BitFlags { bits : $ T, } __impl_bitflags !
2019-10-11T02:15:12.8719684Z     |  ___|_______________________________________-
2019-10-11T02:15:12.8720091Z 11  | |   |     {
2019-10-11T02:15:12.8720497Z 12  | |   |         $ BitFlags : $ T
2019-10-11T02:15:12.8720983Z 13  | |   |         { $ ($ (# [$ inner $ ($ args) *]) * $ Flag = $ value ;) + }
2019-10-11T02:15:12.8721776Z     | |___|_____- in this macro invocation (#3)
2019-10-11T02:15:12.8722165Z 15  |     | } ;
2019-10-11T02:15:12.8722165Z 15  |     | } ;
2019-10-11T02:15:12.8722571Z     |     |___- in this expansion of `__bitflags!` (#2)
2019-10-11T02:15:12.8722862Z     | 
2019-10-11T02:15:12.8723176Z    ::: <::bitflags::bitflags macros>:1:1
2019-10-11T02:15:12.8723433Z     |
2019-10-11T02:15:12.8723878Z 1   |    /  ($ (# [$ outer : meta]) * pub struct $ BitFlags : ident : $ T : ty
2019-10-11T02:15:12.8724238Z 2   |    |   {
2019-10-11T02:15:12.8724617Z 3   |    |       $
2019-10-11T02:15:12.8725582Z 4   |    |       ($ (# [$ inner : ident $ ($ args : tt) *]) * const $ Flag : ident = $
2019-10-11T02:15:12.8725935Z ...      |
2019-10-11T02:15:12.8726309Z 8   |  / |      __bitflags !
2019-10-11T02:15:12.8726720Z 9   |  | |      {
2019-10-11T02:15:12.8727185Z 10  |  | |          $ (# [$ outer]) * (pub) $ BitFlags : $ T
2019-10-11T02:15:12.8727820Z 11  |  | |          { $ ($ (# [$ inner $ ($ args) *]) * $ Flag = $ value ;) + }
2019-10-11T02:15:12.8773575Z     |  |_|______- in this macro invocation (#2)
2019-10-11T02:15:12.8773931Z ...      |
2019-10-11T02:15:12.8774269Z 39  |    |      }
2019-10-11T02:15:12.8774600Z 40  |    |  } ;
2019-10-11T02:15:12.8774600Z 40  |    |  } ;
2019-10-11T02:15:12.8775333Z     |    |____- in this expansion of `bitflags::bitflags!` (#1)
2019-10-11T02:15:12.8775992Z    ::: src/librustc_apfloat/lib.rs:48:1
2019-10-11T02:15:12.8776244Z     |
2019-10-11T02:15:12.8776583Z 48  | /     bitflags::bitflags! {
2019-10-11T02:15:12.8776583Z 48  | /     bitflags::bitflags! {
2019-10-11T02:15:12.8776975Z 49  | |         /// IEEE-754R 7: Default exception handling.
2019-10-11T02:15:12.8777312Z 50  | |         ///
2019-10-11T02:15:12.8777702Z 51  | |         /// UNDERFLOW or OVERFLOW are always returned or-ed with INEXACT.
2019-10-11T02:15:12.8778344Z 60  | |         }
2019-10-11T02:15:12.8778662Z 61  | |     }
2019-10-11T02:15:12.8779031Z     | |_____- in this macro invocation (#1)
2019-10-11T02:15:12.8779124Z 
---
2019-10-11T02:15:14.2458568Z == clock drift check ==
2019-10-11T02:15:14.2477280Z   local time: Fri Oct 11 02:15:14 UTC 2019
2019-10-11T02:15:14.3990238Z   network time: Fri, 11 Oct 2019 02:15:14 GMT
2019-10-11T02:15:14.3995582Z == end clock drift check ==
2019-10-11T02:15:15.3551616Z ##[error]Bash exited with code '1'.
2019-10-11T02:15:15.3604602Z ##[section]Starting: Checkout
2019-10-11T02:15:15.3607259Z ==============================================================================
2019-10-11T02:15:15.3607327Z Task         : Get sources
2019-10-11T02:15:15.3607382Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
