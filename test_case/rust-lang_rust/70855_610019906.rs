plain
2020-04-06T20:24:52.6457995Z ========================== Starting Command Output ===========================
2020-04-06T20:24:52.6460705Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/299c897f-12de-47a8-a8a8-0aeca744daf6.sh
2020-04-06T20:24:52.6460979Z 
2020-04-06T20:24:52.6466619Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T20:24:52.6490404Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-06T20:24:52.6493950Z Task         : Get sources
2020-04-06T20:24:52.6494252Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T20:24:52.6495112Z Version      : 1.0.0
2020-04-06T20:24:52.6495294Z Author       : Microsoft
---
2020-04-06T20:24:53.6375647Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T20:24:53.6380954Z ##[command]git config gc.auto 0
2020-04-06T20:24:53.6384633Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T20:24:53.6387591Z ##[command]git config --get-all http.proxy
2020-04-06T20:24:53.6393968Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70855/merge:refs/remotes/pull/70855/merge
---
2020-04-06T20:26:49.2795863Z  ---> 3fc1b512c57b
2020-04-06T20:26:49.2796129Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T20:26:49.2796549Z  ---> Using cache
2020-04-06T20:26:49.2797674Z  ---> 5ee4295733f4
2020-04-06T20:26:49.2799010Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T20:26:49.2800355Z  ---> 3d07a0fa42fe
2020-04-06T20:26:49.2813487Z Successfully built 3d07a0fa42fe
2020-04-06T20:26:49.2854045Z Successfully tagged rust-ci:latest
2020-04-06T20:26:49.3169879Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T20:26:49.3169879Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T20:26:49.3206577Z Looks like docker image is the same as before, not uploading
2020-04-06T20:26:57.5597156Z [CI_JOB_NAME=mingw-check]
2020-04-06T20:26:57.5834651Z [CI_JOB_NAME=mingw-check]
2020-04-06T20:26:57.5861885Z == clock drift check ==
2020-04-06T20:26:57.5871327Z   local time: Mon Apr  6 20:26:57 UTC 2020
2020-04-06T20:26:57.6814409Z   network time: Mon, 06 Apr 2020 20:26:57 GMT
2020-04-06T20:26:57.6840028Z Starting sccache server...
2020-04-06T20:26:57.7715861Z configure: processing command line
2020-04-06T20:26:57.7716306Z configure: 
2020-04-06T20:26:57.7717180Z configure: rust.parallel-compiler := True
---
2020-04-06T20:28:41.1701314Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-04-06T20:28:43.8557308Z error[E0412]: cannot find type `Ordering` in this scope
2020-04-06T20:28:43.8557926Z    --> src/libcore/num/mod.rs:120:61
2020-04-06T20:28:43.8558357Z     |
2020-04-06T20:28:43.8558899Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:43.8559687Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:43.8561277Z 47  | |             doc_comment! {
2020-04-06T20:28:43.8561821Z ...   |
2020-04-06T20:28:43.8561821Z ...   |
2020-04-06T20:28:43.8562547Z 120 | |                 fn partial_cmp(&self, rhs: &$Int) -> Option<Ordering> {
2020-04-06T20:28:43.8564337Z ...   |
2020-04-06T20:28:43.8565048Z 129 | |     }
2020-04-06T20:28:43.8565811Z 130 | | }
2020-04-06T20:28:43.8566471Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:43.8566471Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:43.8567029Z 131 | 
2020-04-06T20:28:43.8567593Z 132 | / nonzero_integers! {
2020-04-06T20:28:43.8568349Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:43.8569508Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:43.8570500Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:43.8571523Z ...   |
2020-04-06T20:28:43.8572580Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:43.8574228Z     | |_- in this macro invocation
2020-04-06T20:28:43.8574796Z     |
2020-04-06T20:28:43.8575447Z help: possible candidates are found in other modules, you can import them into scope
2020-04-06T20:28:43.8576053Z     |
---
2020-04-06T20:28:43.8578900Z 
2020-04-06T20:28:47.3966520Z error[E0308]: mismatched types
2020-04-06T20:28:47.3967214Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.3967684Z     |
2020-04-06T20:28:47.3968325Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.3969247Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.3971466Z 47  | |             doc_comment! {
2020-04-06T20:28:47.3972132Z ...   |
2020-04-06T20:28:47.3972811Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.3973764Z     | |                               ^^^ expected `u8`, found `&u8`
2020-04-06T20:28:47.3973764Z     | |                               ^^^ expected `u8`, found `&u8`
2020-04-06T20:28:47.3974466Z ...   |
2020-04-06T20:28:47.3975103Z 129 | |     }
2020-04-06T20:28:47.3975779Z 130 | | }
2020-04-06T20:28:47.3976724Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.3977330Z 131 | 
2020-04-06T20:28:47.3977952Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.3978813Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.3980204Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.3981253Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.3982000Z ...   |
2020-04-06T20:28:47.3983283Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.3984816Z     | |_- in this macro invocation
2020-04-06T20:28:47.3985132Z 
2020-04-06T20:28:47.4050551Z error[E0308]: mismatched types
2020-04-06T20:28:47.4051216Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4051216Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4051753Z     |
2020-04-06T20:28:47.4054350Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4055721Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4057409Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4058085Z ...   |
2020-04-06T20:28:47.4058816Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4060207Z     | |                               ^^^ expected `u16`, found `&u16`
2020-04-06T20:28:47.4060207Z     | |                               ^^^ expected `u16`, found `&u16`
2020-04-06T20:28:47.4060919Z ...   |
2020-04-06T20:28:47.4061635Z 129 | |     }
2020-04-06T20:28:47.4062507Z 130 | | }
2020-04-06T20:28:47.4063302Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4063918Z 131 | 
2020-04-06T20:28:47.4064588Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4065451Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4066611Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4067624Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4068509Z ...   |
2020-04-06T20:28:47.4069393Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4071010Z     | |_- in this macro invocation
2020-04-06T20:28:47.4071353Z 
2020-04-06T20:28:47.4144947Z error[E0308]: mismatched types
2020-04-06T20:28:47.4145505Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4145505Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4145975Z     |
2020-04-06T20:28:47.4146557Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4147379Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4148783Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4149355Z ...   |
2020-04-06T20:28:47.4150142Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4151411Z     | |                               ^^^ expected `u32`, found `&u32`
2020-04-06T20:28:47.4151411Z     | |                               ^^^ expected `u32`, found `&u32`
2020-04-06T20:28:47.4152525Z ...   |
2020-04-06T20:28:47.4153704Z 129 | |     }
2020-04-06T20:28:47.4154604Z 130 | | }
2020-04-06T20:28:47.4155668Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4156260Z 131 | 
2020-04-06T20:28:47.4156898Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4157738Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4158889Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4160188Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4160975Z ...   |
2020-04-06T20:28:47.4161887Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4164278Z     | |_- in this macro invocation
2020-04-06T20:28:47.4164617Z 
2020-04-06T20:28:47.4242640Z error[E0308]: mismatched types
2020-04-06T20:28:47.4243787Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4243787Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4244270Z     |
2020-04-06T20:28:47.4244881Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4246448Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4248702Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4249718Z ...   |
2020-04-06T20:28:47.4250480Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4251483Z     | |                               ^^^ expected `u64`, found `&u64`
2020-04-06T20:28:47.4251483Z     | |                               ^^^ expected `u64`, found `&u64`
2020-04-06T20:28:47.4252266Z ...   |
2020-04-06T20:28:47.4252923Z 129 | |     }
2020-04-06T20:28:47.4253646Z 130 | | }
2020-04-06T20:28:47.4254994Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4255798Z 131 | 
2020-04-06T20:28:47.4256645Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4267326Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4268396Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4269462Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4270194Z ...   |
2020-04-06T20:28:47.4271197Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4273740Z     | |_- in this macro invocation
2020-04-06T20:28:47.4274069Z 
2020-04-06T20:28:47.4360428Z error[E0308]: mismatched types
2020-04-06T20:28:47.4361088Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4361088Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4361612Z     |
2020-04-06T20:28:47.4362302Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4363256Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4366926Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4367753Z ...   |
2020-04-06T20:28:47.4368526Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4369532Z     | |                               ^^^ expected `u128`, found `&u128`
2020-04-06T20:28:47.4369532Z     | |                               ^^^ expected `u128`, found `&u128`
2020-04-06T20:28:47.4370596Z ...   |
2020-04-06T20:28:47.4371339Z 129 | |     }
2020-04-06T20:28:47.4371963Z 130 | | }
2020-04-06T20:28:47.4372679Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4373273Z 131 | 
2020-04-06T20:28:47.4373872Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4374706Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4375594Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4376504Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4377140Z ...   |
2020-04-06T20:28:47.4377888Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4379528Z     | |_- in this macro invocation
2020-04-06T20:28:47.4379807Z 
2020-04-06T20:28:47.4464159Z error[E0308]: mismatched types
2020-04-06T20:28:47.4464907Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4464907Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4465374Z     |
2020-04-06T20:28:47.4465981Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4466810Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4468491Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4469073Z ...   |
2020-04-06T20:28:47.4469710Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4469710Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4470582Z     | |                               ^^^ expected `usize`, found `&usize`
2020-04-06T20:28:47.4471839Z 129 | |     }
2020-04-06T20:28:47.4472501Z 130 | | }
2020-04-06T20:28:47.4473211Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4473786Z 131 | 
2020-04-06T20:28:47.4473786Z 131 | 
2020-04-06T20:28:47.4474409Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4475220Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4476335Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4477505Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4478236Z ...   |
2020-04-06T20:28:47.4479221Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4480870Z     | |_- in this macro invocation
2020-04-06T20:28:47.4481162Z 
2020-04-06T20:28:47.4550126Z error[E0308]: mismatched types
2020-04-06T20:28:47.4550952Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4550952Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4551773Z     |
2020-04-06T20:28:47.4552465Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4553588Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4555392Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4556001Z ...   |
2020-04-06T20:28:47.4556698Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4557625Z     | |                               ^^^ expected `i8`, found `&i8`
2020-04-06T20:28:47.4557625Z     | |                               ^^^ expected `i8`, found `&i8`
2020-04-06T20:28:47.4558404Z ...   |
2020-04-06T20:28:47.4559072Z 129 | |     }
2020-04-06T20:28:47.4559746Z 130 | | }
2020-04-06T20:28:47.4560494Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4561134Z 131 | 
2020-04-06T20:28:47.4561777Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4562661Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4563628Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4564590Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4565445Z ...   |
2020-04-06T20:28:47.4566214Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4567968Z     | |_- in this macro invocation
2020-04-06T20:28:47.4568444Z 
2020-04-06T20:28:47.4652624Z error[E0308]: mismatched types
2020-04-06T20:28:47.4653499Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4653499Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4653971Z     |
2020-04-06T20:28:47.4654621Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4655509Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4657080Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4657682Z ...   |
2020-04-06T20:28:47.4658359Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4659809Z     | |                               ^^^ expected `i16`, found `&i16`
2020-04-06T20:28:47.4659809Z     | |                               ^^^ expected `i16`, found `&i16`
2020-04-06T20:28:47.4660537Z ...   |
2020-04-06T20:28:47.4661181Z 129 | |     }
2020-04-06T20:28:47.4661885Z 130 | | }
2020-04-06T20:28:47.4662821Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4663497Z 131 | 
2020-04-06T20:28:47.4664163Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4665347Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4666452Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4667484Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4668231Z ...   |
2020-04-06T20:28:47.4669097Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4671457Z     | |_- in this macro invocation
2020-04-06T20:28:47.4671786Z 
2020-04-06T20:28:47.4761392Z error[E0308]: mismatched types
2020-04-06T20:28:47.4762072Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4762072Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4762560Z     |
2020-04-06T20:28:47.4763218Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4764164Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4765781Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4766438Z ...   |
2020-04-06T20:28:47.4767145Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4768126Z     | |                               ^^^ expected `i32`, found `&i32`
2020-04-06T20:28:47.4768126Z     | |                               ^^^ expected `i32`, found `&i32`
2020-04-06T20:28:47.4768847Z ...   |
2020-04-06T20:28:47.4769477Z 129 | |     }
2020-04-06T20:28:47.4770197Z 130 | | }
2020-04-06T20:28:47.4770974Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4771626Z 131 | 
2020-04-06T20:28:47.4772658Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4773528Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4774509Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4775461Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4776134Z ...   |
2020-04-06T20:28:47.4776954Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4778601Z     | |_- in this macro invocation
2020-04-06T20:28:47.4778902Z 
2020-04-06T20:28:47.4850957Z error[E0308]: mismatched types
2020-04-06T20:28:47.4851714Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4851714Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4852148Z     |
2020-04-06T20:28:47.4852723Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4853542Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4855337Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4856511Z ...   |
2020-04-06T20:28:47.4857243Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4858549Z     | |                               ^^^ expected `i64`, found `&i64`
2020-04-06T20:28:47.4858549Z     | |                               ^^^ expected `i64`, found `&i64`
2020-04-06T20:28:47.4859653Z ...   |
2020-04-06T20:28:47.4860310Z 129 | |     }
2020-04-06T20:28:47.4861059Z 130 | | }
2020-04-06T20:28:47.4861879Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4862707Z 131 | 
2020-04-06T20:28:47.4863432Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4864970Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4866076Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4867004Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4867988Z ...   |
2020-04-06T20:28:47.4868770Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4870176Z     | |_- in this macro invocation
2020-04-06T20:28:47.4870473Z 
2020-04-06T20:28:47.4936659Z error[E0308]: mismatched types
2020-04-06T20:28:47.4937469Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4937469Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.4938124Z     |
2020-04-06T20:28:47.4938885Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.4940415Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.4943309Z 47  | |             doc_comment! {
2020-04-06T20:28:47.4944028Z ...   |
2020-04-06T20:28:47.4944793Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.4945816Z     | |                               ^^^ expected `i128`, found `&i128`
2020-04-06T20:28:47.4945816Z     | |                               ^^^ expected `i128`, found `&i128`
2020-04-06T20:28:47.4946606Z ...   |
2020-04-06T20:28:47.4947532Z 129 | |     }
2020-04-06T20:28:47.4948719Z 130 | | }
2020-04-06T20:28:47.4949849Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.4950650Z 131 | 
2020-04-06T20:28:47.4952003Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.4952956Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.4954028Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.4955095Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.4955915Z ...   |
2020-04-06T20:28:47.4956808Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.4958556Z     | |_- in this macro invocation
2020-04-06T20:28:47.4958971Z 
2020-04-06T20:28:47.5013676Z error[E0308]: mismatched types
2020-04-06T20:28:47.5014677Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.5014677Z    --> src/libcore/num/mod.rs:115:31
2020-04-06T20:28:47.5015984Z     |
2020-04-06T20:28:47.5016865Z 44  | / macro_rules! nonzero_integers {
2020-04-06T20:28:47.5018010Z 45  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T20:28:47.5020048Z 47  | |             doc_comment! {
2020-04-06T20:28:47.5020863Z ...   |
2020-04-06T20:28:47.5021754Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.5021754Z 115 | |                     self.0 == rhs
2020-04-06T20:28:47.5023229Z     | |                               ^^^ expected `isize`, found `&isize`
2020-04-06T20:28:47.5025100Z 129 | |     }
2020-04-06T20:28:47.5026039Z 130 | | }
2020-04-06T20:28:47.5027241Z     | |_- in this expansion of `nonzero_integers!`
2020-04-06T20:28:47.5028028Z 131 | 
2020-04-06T20:28:47.5028028Z 131 | 
2020-04-06T20:28:47.5028861Z 132 | / nonzero_integers! {
2020-04-06T20:28:47.5030125Z 133 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T20:28:47.5031325Z 134 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T20:28:47.5032688Z 135 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T20:28:47.5033546Z ...   |
2020-04-06T20:28:47.5034783Z 144 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T20:28:47.5037121Z     | |_- in this macro invocation
2020-04-06T20:28:47.5037578Z 
2020-04-06T20:28:49.2017633Z    Compiling libc v0.2.66
2020-04-06T20:28:51.3830152Z    Compiling autocfg v0.1.7
---
2020-04-06T20:28:52.6039993Z expected success, got: exit code: 101
2020-04-06T20:28:52.6050792Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-06T20:28:52.6051297Z Build completed unsuccessfully in 0:01:54
2020-04-06T20:28:52.6113532Z == clock drift check ==
2020-04-06T20:28:52.7040726Z   local time: Mon Apr  6 20:28:52 UTC 2020
2020-04-06T20:28:54.4067167Z   network time: Mon, 06 Apr 2020 20:28:54 GMT
2020-04-06T20:28:56.7319801Z 
2020-04-06T20:28:56.7319801Z 
2020-04-06T20:28:56.7397105Z ##[error]Bash exited with code '1'.
2020-04-06T20:28:56.7413257Z ##[section]Finishing: Run build
2020-04-06T20:28:56.7473372Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-06T20:28:56.7478313Z Task         : Get sources
2020-04-06T20:28:56.7478692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T20:28:56.7479022Z Version      : 1.0.0
2020-04-06T20:28:56.7479273Z Author       : Microsoft
2020-04-06T20:28:56.7479273Z Author       : Microsoft
2020-04-06T20:28:56.7479643Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T20:28:56.7480069Z ==============================================================================
2020-04-06T20:28:57.1178399Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T20:28:57.1231297Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-06T20:28:57.1326321Z Cleaning up task key
2020-04-06T20:28:57.1328173Z Start cleaning up orphan processes.
2020-04-06T20:28:57.1558197Z Terminate orphan process: pid (3431) (python)
2020-04-06T20:28:57.1703684Z ##[section]Finishing: Finalize Job
