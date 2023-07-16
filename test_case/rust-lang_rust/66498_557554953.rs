plain
2019-11-22T14:30:27.8458381Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-22T14:30:27.8472351Z ##[command]git config gc.auto 0
2019-11-22T14:30:27.8475315Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-22T14:30:27.8480056Z ##[command]git config --get-all http.proxy
2019-11-22T14:30:27.8484501Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
---
2019-11-22T14:36:10.4406126Z      |
2019-11-22T14:36:10.4406383Z 4725 | impl From<!> for TryFromIntError {
2019-11-22T14:36:10.4406788Z      |           ^
2019-11-22T14:36:10.4406966Z      |
2019-11-22T14:36:10.4407325Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4407854Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4428936Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4429183Z     --> src/libcore/num/mod.rs:4726:20
2019-11-22T14:36:10.4429365Z      |
2019-11-22T14:36:10.4429365Z      |
2019-11-22T14:36:10.4429611Z 4726 |     fn from(never: !) -> TryFromIntError {
2019-11-22T14:36:10.4429992Z      |
2019-11-22T14:36:10.4429992Z      |
2019-11-22T14:36:10.4430302Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4430557Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4547472Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4547721Z    --> src/libcore/marker.rs:778:19
2019-11-22T14:36:10.4547918Z     |
2019-11-22T14:36:10.4547918Z     |
2019-11-22T14:36:10.4548129Z 778 |     impl Copy for ! {}
2019-11-22T14:36:10.4548524Z     |
2019-11-22T14:36:10.4548524Z     |
2019-11-22T14:36:10.4548815Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4549074Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4711489Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4712121Z     --> src/libcore/cmp.rs:1132:24
2019-11-22T14:36:10.4712517Z      |
2019-11-22T14:36:10.4713280Z 1132 |     impl PartialEq for ! {
2019-11-22T14:36:10.4713280Z 1132 |     impl PartialEq for ! {
2019-11-22T14:36:10.4713547Z      |                        ^
2019-11-22T14:36:10.4713756Z      |
2019-11-22T14:36:10.4714110Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4714575Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4743010Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4743522Z     --> src/libcore/cmp.rs:1133:26
2019-11-22T14:36:10.4743761Z      |
2019-11-22T14:36:10.4743761Z      |
2019-11-22T14:36:10.4744158Z 1133 |         fn eq(&self, _: &!) -> bool {
2019-11-22T14:36:10.4744614Z      |
2019-11-22T14:36:10.4744614Z      |
2019-11-22T14:36:10.4745150Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4745420Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4774391Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4774870Z     --> src/libcore/cmp.rs:1139:17
2019-11-22T14:36:10.4775082Z      |
2019-11-22T14:36:10.4775082Z      |
2019-11-22T14:36:10.4775514Z 1139 |     impl Eq for ! {}
2019-11-22T14:36:10.4775901Z      |
2019-11-22T14:36:10.4775901Z      |
2019-11-22T14:36:10.4776421Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4776696Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4804839Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4805148Z     --> src/libcore/cmp.rs:1142:25
2019-11-22T14:36:10.4805358Z      |
2019-11-22T14:36:10.4805590Z 1142 |     impl PartialOrd for ! {
2019-11-22T14:36:10.4805590Z 1142 |     impl PartialOrd for ! {
2019-11-22T14:36:10.4805833Z      |                         ^
2019-11-22T14:36:10.4806209Z      |
2019-11-22T14:36:10.4807898Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4808246Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4890508Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4891346Z     --> src/libcore/cmp.rs:1143:35
2019-11-22T14:36:10.4891574Z      |
2019-11-22T14:36:10.4891574Z      |
2019-11-22T14:36:10.4892016Z 1143 |         fn partial_cmp(&self, _: &!) -> Option<Ordering> {
2019-11-22T14:36:10.4892732Z      |
2019-11-22T14:36:10.4892732Z      |
2019-11-22T14:36:10.4893076Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4893593Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4896933Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4897461Z     --> src/libcore/cmp.rs:1149:18
2019-11-22T14:36:10.4897645Z      |
2019-11-22T14:36:10.4898109Z 1149 |     impl Ord for ! {
2019-11-22T14:36:10.4898109Z 1149 |     impl Ord for ! {
2019-11-22T14:36:10.4898491Z      |                  ^
2019-11-22T14:36:10.4898706Z      |
2019-11-22T14:36:10.4899236Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4899979Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4903631Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4903900Z     --> src/libcore/cmp.rs:1150:27
2019-11-22T14:36:10.4904073Z      |
2019-11-22T14:36:10.4904073Z      |
2019-11-22T14:36:10.4904325Z 1150 |         fn cmp(&self, _: &!) -> Ordering {
2019-11-22T14:36:10.4904705Z      |
2019-11-22T14:36:10.4904705Z      |
2019-11-22T14:36:10.4905010Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4905653Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4911209Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4911518Z    --> src/libcore/clone.rs:189:20
2019-11-22T14:36:10.4911760Z     |
2019-11-22T14:36:10.4912016Z 189 |     impl Clone for ! {
2019-11-22T14:36:10.4912016Z 189 |     impl Clone for ! {
2019-11-22T14:36:10.4912265Z     |                    ^
2019-11-22T14:36:10.4912490Z     |
2019-11-22T14:36:10.4912816Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4913114Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4925908Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4926312Z    --> src/libcore/convert.rs:557:14
2019-11-22T14:36:10.4926492Z     |
2019-11-22T14:36:10.4926821Z 557 | impl<T> From<!> for T {
2019-11-22T14:36:10.4926821Z 557 | impl<T> From<!> for T {
2019-11-22T14:36:10.4927051Z     |              ^
2019-11-22T14:36:10.4927221Z     |
2019-11-22T14:36:10.4927514Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4927791Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4928017Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4928254Z    --> src/libcore/convert.rs:558:16
2019-11-22T14:36:10.4928423Z     |
2019-11-22T14:36:10.4928423Z     |
2019-11-22T14:36:10.4928633Z 558 |     fn from(t: !) -> T { t }
2019-11-22T14:36:10.4929022Z     |
2019-11-22T14:36:10.4929022Z     |
2019-11-22T14:36:10.4929304Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4929555Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.4972148Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.4972580Z    --> src/libcore/convert.rs:643:23
2019-11-22T14:36:10.4972820Z     |
2019-11-22T14:36:10.4973187Z 643 | pub type Infallible = !;
2019-11-22T14:36:10.4973187Z 643 | pub type Infallible = !;
2019-11-22T14:36:10.4973448Z     |                       ^
2019-11-22T14:36:10.4973616Z     |
2019-11-22T14:36:10.4973924Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.4974179Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.5025451Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.5025709Z     --> src/libcore/iter/traits/iterator.rs:1828:84
2019-11-22T14:36:10.5025888Z      |
2019-11-22T14:36:10.5025888Z      |
2019-11-22T14:36:10.5026249Z 1828 |         fn ok<B, T>(mut f: impl FnMut(B, T) -> B) -> impl FnMut(B, T) -> Result<B, !> {
2019-11-22T14:36:10.5027069Z      |
2019-11-22T14:36:10.5027069Z      |
2019-11-22T14:36:10.5027614Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.5028069Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.5064208Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.5064461Z    --> src/libcore/iter/traits/double_ended.rs:228:84
2019-11-22T14:36:10.5064709Z     |
2019-11-22T14:36:10.5064709Z     |
2019-11-22T14:36:10.5064987Z 228 |         fn ok<B, T>(mut f: impl FnMut(B, T) -> B) -> impl FnMut(B, T) -> Result<B, !> {
2019-11-22T14:36:10.5065451Z     |
2019-11-22T14:36:10.5065451Z     |
2019-11-22T14:36:10.5065738Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.5066390Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.5238246Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.5238518Z    --> src/libcore/hash/mod.rs:616:19
2019-11-22T14:36:10.5238738Z     |
2019-11-22T14:36:10.5238947Z 616 |     impl Hash for ! {
2019-11-22T14:36:10.5238947Z 616 |     impl Hash for ! {
2019-11-22T14:36:10.5239212Z     |                   ^
2019-11-22T14:36:10.5239380Z     |
2019-11-22T14:36:10.5239686Z     = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.5240195Z     = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.5299864Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.5300107Z     --> src/libcore/fmt/mod.rs:1939:16
2019-11-22T14:36:10.5300330Z      |
2019-11-22T14:36:10.5300568Z 1939 | impl Debug for ! {
2019-11-22T14:36:10.5300568Z 1939 | impl Debug for ! {
2019-11-22T14:36:10.5300776Z      |                ^
2019-11-22T14:36:10.5301103Z      |
2019-11-22T14:36:10.5301463Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.5302119Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:10.5331634Z error[E0658]: The `!` type is experimental
2019-11-22T14:36:10.5331980Z     --> src/libcore/fmt/mod.rs:1946:18
2019-11-22T14:36:10.5332182Z      |
2019-11-22T14:36:10.5332393Z 1946 | impl Display for ! {
2019-11-22T14:36:10.5332393Z 1946 | impl Display for ! {
2019-11-22T14:36:10.5332599Z      |                  ^
2019-11-22T14:36:10.5332780Z      |
2019-11-22T14:36:10.5333270Z      = note: for more information, see ***/issues/35121
2019-11-22T14:36:10.5333533Z      = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-11-22T14:36:13.2458796Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-11-22T14:36:14.1348362Z    Compiling libc v0.2.64
2019-11-22T14:36:14.6988912Z    Compiling autocfg v0.1.6
2019-11-22T14:36:15.6350441Z    Compiling std v0.0.0 (/checkout/src/libstd)
---
2019-11-22T14:36:17.7826025Z   local time: Fri Nov 22 14:36:17 UTC 2019
2019-11-22T14:36:17.8200544Z   network time: Fri, 22 Nov 2019 14:36:17 GMT
2019-11-22T14:36:17.8206996Z == end clock drift check ==
2019-11-22T14:36:19.2871412Z 
2019-11-22T14:36:19.2955082Z ##[error]Bash exited with code '1'.
2019-11-22T14:36:19.2983021Z ##[section]Starting: Checkout
2019-11-22T14:36:19.2984907Z ==============================================================================
2019-11-22T14:36:19.2984954Z Task         : Get sources
2019-11-22T14:36:19.2985013Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
