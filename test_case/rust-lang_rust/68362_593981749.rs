plain
2020-03-03T14:22:42.9517113Z ========================== Starting Command Output ===========================
2020-03-03T14:22:42.9520379Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/81a4671a-f084-4306-b57e-0f6fbbae2277.sh
2020-03-03T14:22:42.9520683Z 
2020-03-03T14:22:42.9524641Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T14:22:42.9545931Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T14:22:42.9549499Z Task         : Get sources
2020-03-03T14:22:42.9549833Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T14:22:42.9550167Z Version      : 1.0.0
2020-03-03T14:22:42.9550382Z Author       : Microsoft
---
2020-03-03T14:22:44.1889356Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T14:22:44.1897125Z ##[command]git config gc.auto 0
2020-03-03T14:22:44.1953952Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T14:22:44.1957818Z ##[command]git config --get-all http.proxy
2020-03-03T14:22:44.1965563Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-03-03T14:30:04.6883720Z     |
2020-03-03T14:30:04.6885161Z 126 |     PartialOrd,
2020-03-03T14:30:04.6886918Z     |     ---------- in this macro invocation
2020-03-03T14:30:04.6888327Z ...
2020-03-03T14:30:04.6891072Z 214 |     GeneratorWitness(Binder<&'tcx List<Ty<'tcx>>>, &'tcx List<ty::Predicate<'tcx>>),
2020-03-03T14:30:04.6894031Z     |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `ty::Predicate<'_> < ty::Predicate<'_>` and `ty::Predicate<'_> > ty::Predicate<'_>`
2020-03-03T14:30:04.6897321Z    ::: <::core::cmp::PartialOrd macros>:1:1
2020-03-03T14:30:04.6898529Z     |
2020-03-03T14:30:04.6899825Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.6901905Z     | ---------------------- in this expansion of `#[derive(PartialOrd)]`
2020-03-03T14:30:04.6901905Z     | ---------------------- in this expansion of `#[derive(PartialOrd)]`
2020-03-03T14:30:04.6903337Z     |
2020-03-03T14:30:04.6904903Z     = help: the trait `std::cmp::PartialOrd` is not implemented for `ty::Predicate<'_>`
2020-03-03T14:30:04.6907091Z     = note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.6910061Z     = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&ty::List<ty::Predicate<'_>>>` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.6913037Z 
2020-03-03T14:30:04.7443309Z error[E0277]: the trait bound `ty::Predicate<'_>: std::cmp::Ord` is not satisfied
2020-03-03T14:30:04.7447900Z    --> src/librustc/ty/sty.rs:214:52
2020-03-03T14:30:04.7448809Z     |
2020-03-03T14:30:04.7448809Z     |
2020-03-03T14:30:04.7449778Z 127 |     Ord,
2020-03-03T14:30:04.7451070Z     |     --- in this macro invocation
2020-03-03T14:30:04.7452052Z ...
2020-03-03T14:30:04.7453433Z 214 |     GeneratorWitness(Binder<&'tcx List<Ty<'tcx>>>, &'tcx List<ty::Predicate<'tcx>>),
2020-03-03T14:30:04.7457407Z     | 
2020-03-03T14:30:04.7458405Z    ::: <::core::cmp::Ord macros>:1:1
2020-03-03T14:30:04.7459325Z     |
2020-03-03T14:30:04.7460406Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.7460406Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.7461274Z     | ---------------------- in this expansion of `#[derive(Ord)]`
2020-03-03T14:30:04.7462780Z     = note: required because of the requirements on the impl of `std::cmp::Ord` for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.7462780Z     = note: required because of the requirements on the impl of `std::cmp::Ord` for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.7463930Z     = note: required because of the requirements on the impl of `std::cmp::Ord` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.7473255Z 
2020-03-03T14:30:04.8006630Z error[E0277]: the size for values of type `ty::OpaqueListContents` cannot be known at compilation time
2020-03-03T14:30:04.8008089Z    --> src/librustc/ty/sty.rs:214:52
2020-03-03T14:30:04.8008975Z     |
2020-03-03T14:30:04.8008975Z     |
2020-03-03T14:30:04.8009954Z 130 |     RustcDecodable,
2020-03-03T14:30:04.8011379Z     |     -------------- in this macro invocation
2020-03-03T14:30:04.8012382Z ...
2020-03-03T14:30:04.8013751Z 214 |     GeneratorWitness(Binder<&'tcx List<Ty<'tcx>>>, &'tcx List<ty::Predicate<'tcx>>),
2020-03-03T14:30:04.8017390Z     | 
2020-03-03T14:30:04.8018527Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2020-03-03T14:30:04.8019533Z     |
2020-03-03T14:30:04.8020576Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.8020576Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.8022322Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8023441Z     |
2020-03-03T14:30:04.8025006Z     = help: within `ty::List<ty::Predicate<'_>>`, the trait `std::marker::Sized` is not implemented for `ty::OpaqueListContents`
2020-03-03T14:30:04.8029418Z     = note: required because it appears within the type `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8029418Z     = note: required because it appears within the type `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8031473Z     = note: required because of the requirements on the impl of `serialize::serialize::UseSpecializedDecodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8033648Z     = note: required because of the requirements on the impl of `serialize::serialize::Decodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8035435Z     = note: required by `serialize::serialize::Decodable::decode`
2020-03-03T14:30:04.8049408Z 
2020-03-03T14:30:04.8051120Z error[E0277]: the trait bound `ty::List<ty::Predicate<'_>>: serialize::serialize::UseSpecializedDecodable` is not satisfied
2020-03-03T14:30:04.8052568Z    --> src/librustc/ty/sty.rs:214:52
2020-03-03T14:30:04.8054400Z 130 |     RustcDecodable,
2020-03-03T14:30:04.8055786Z     |     -------------- in this macro invocation
2020-03-03T14:30:04.8056794Z ...
2020-03-03T14:30:04.8056794Z ...
2020-03-03T14:30:04.8058177Z 214 |     GeneratorWitness(Binder<&'tcx List<Ty<'tcx>>>, &'tcx List<ty::Predicate<'tcx>>),
2020-03-03T14:30:04.8060784Z     |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `serialize::serialize::UseSpecializedDecodable` is not implemented for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8063637Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2020-03-03T14:30:04.8064644Z     |
2020-03-03T14:30:04.8065711Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.8067266Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8067266Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8068386Z     |
2020-03-03T14:30:04.8069965Z     = help: the following implementations were found:
2020-03-03T14:30:04.8071408Z               <&'tcx ty::List<&'tcx ty::TyS<'tcx>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8072923Z               <&'tcx ty::List<infer::canonical::CanonicalVarInfo> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8073982Z               <&'tcx ty::List<ty::OutlivesPredicate<&'tcx ty::sty::RegionKind, &'tcx ty::sty::RegionKind>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8074983Z               <&'tcx ty::List<ty::sty::ExistentialPredicate<'tcx>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8075969Z               <&'tcx ty::List<ty::subst::GenericArg<'tcx>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8077057Z     = note: required because of the requirements on the impl of `serialize::serialize::Decodable` for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8078367Z     = note: required because of the requirements on the impl of `serialize::serialize::UseSpecializedDecodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8079647Z     = note: required because of the requirements on the impl of `serialize::serialize::Decodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8080683Z     = note: required by `serialize::serialize::Decodable::decode`
2020-03-03T14:30:04.8089242Z error[E0277]: the size for values of type `ty::OpaqueListContents` cannot be known at compilation time
2020-03-03T14:30:04.8090039Z    --> src/librustc/ty/sty.rs:130:5
2020-03-03T14:30:04.8090568Z     |
2020-03-03T14:30:04.8091131Z 130 |     RustcDecodable,
2020-03-03T14:30:04.8091131Z 130 |     RustcDecodable,
2020-03-03T14:30:04.8091773Z     |     ^^^^^^^^^^^^^^
2020-03-03T14:30:04.8092397Z     |     |
2020-03-03T14:30:04.8093095Z     |     doesn't have a size known at compile-time
2020-03-03T14:30:04.8093835Z     |     in this macro invocation
2020-03-03T14:30:04.8094366Z     | 
2020-03-03T14:30:04.8094974Z    ::: /checkout/src/libserialize/serialize.rs:388:18
2020-03-03T14:30:04.8095545Z     |
2020-03-03T14:30:04.8096289Z 388 |     fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
2020-03-03T14:30:04.8097447Z     |                  ------- required by this bound in `serialize::serialize::Decodable::decode`
2020-03-03T14:30:04.8098892Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2020-03-03T14:30:04.8099470Z     |
2020-03-03T14:30:04.8100093Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.8100996Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8100996Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8101650Z     |
2020-03-03T14:30:04.8102563Z     = help: within `ty::List<ty::Predicate<'_>>`, the trait `std::marker::Sized` is not implemented for `ty::OpaqueListContents`
2020-03-03T14:30:04.8104967Z     = note: required because it appears within the type `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8104967Z     = note: required because it appears within the type `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8106169Z     = note: required because of the requirements on the impl of `serialize::serialize::UseSpecializedDecodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8107437Z     = note: required because of the requirements on the impl of `serialize::serialize::Decodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8108030Z 
2020-03-03T14:30:04.8135255Z error[E0277]: the trait bound `ty::List<ty::Predicate<'_>>: serialize::serialize::UseSpecializedDecodable` is not satisfied
2020-03-03T14:30:04.8136132Z    --> src/librustc/ty/sty.rs:130:5
2020-03-03T14:30:04.8137205Z 130 |     RustcDecodable,
2020-03-03T14:30:04.8137880Z     |     ^^^^^^^^^^^^^^
2020-03-03T14:30:04.8138491Z     |     |
2020-03-03T14:30:04.8138491Z     |     |
2020-03-03T14:30:04.8139454Z     |     the trait `serialize::serialize::UseSpecializedDecodable` is not implemented for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8140931Z     | 
2020-03-03T14:30:04.8141539Z    ::: /checkout/src/libserialize/serialize.rs:388:18
2020-03-03T14:30:04.8142109Z     |
2020-03-03T14:30:04.8142109Z     |
2020-03-03T14:30:04.8142852Z 388 |     fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
2020-03-03T14:30:04.8144027Z     |                  ------- required by this bound in `serialize::serialize::Decodable::decode`
2020-03-03T14:30:04.8145469Z    ::: <::core::macros::builtin::RustcDecodable macros>:1:1
2020-03-03T14:30:04.8146065Z     |
2020-03-03T14:30:04.8146672Z 1   | ($ item : item) => { }
2020-03-03T14:30:04.8147580Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8147580Z     | ---------------------- in this expansion of `#[derive(RustcDecodable)]`
2020-03-03T14:30:04.8148237Z     |
2020-03-03T14:30:04.8148897Z     = help: the following implementations were found:
2020-03-03T14:30:04.8150205Z               <&'tcx ty::List<&'tcx ty::TyS<'tcx>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8151108Z               <&'tcx ty::List<infer::canonical::CanonicalVarInfo> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8152153Z               <&'tcx ty::List<ty::OutlivesPredicate<&'tcx ty::sty::RegionKind, &'tcx ty::sty::RegionKind>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8153157Z               <&'tcx ty::List<ty::sty::ExistentialPredicate<'tcx>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8154064Z               <&'tcx ty::List<ty::subst::GenericArg<'tcx>> as serialize::serialize::UseSpecializedDecodable>
2020-03-03T14:30:04.8155155Z     = note: required because of the requirements on the impl of `serialize::serialize::Decodable` for `ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8156579Z     = note: required because of the requirements on the impl of `serialize::serialize::UseSpecializedDecodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:04.8157878Z     = note: required because of the requirements on the impl of `serialize::serialize::Decodable` for `&ty::List<ty::Predicate<'_>>`
2020-03-03T14:30:05.3459371Z error: aborting due to 6 previous errors
2020-03-03T14:30:05.3460221Z 
2020-03-03T14:30:05.3470797Z For more information about this error, try `rustc --explain E0277`.
2020-03-03T14:30:05.3705254Z error: could not compile `rustc`.
2020-03-03T14:30:05.3705254Z error: could not compile `rustc`.
2020-03-03T14:30:05.3705561Z 
2020-03-03T14:30:05.3705995Z To learn more, run the command again with --verbose.
2020-03-03T14:30:05.3761694Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-03T14:30:05.3773954Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-03T14:30:05.3774341Z Build completed unsuccessfully in 0:04:46
2020-03-03T14:30:05.3823141Z == clock drift check ==
2020-03-03T14:30:05.3832168Z   local time: Tue Mar  3 14:30:05 UTC 2020
2020-03-03T14:30:05.3832168Z   local time: Tue Mar  3 14:30:05 UTC 2020
2020-03-03T14:30:05.9271796Z   network time: Tue, 03 Mar 2020 14:30:05 GMT
2020-03-03T14:30:05.9284021Z == end clock drift check ==
2020-03-03T14:30:06.3776104Z 
2020-03-03T14:30:06.3855504Z ##[error]Bash exited with code '1'.
2020-03-03T14:30:06.3869935Z ##[section]Finishing: Run build
2020-03-03T14:30:06.3917290Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T14:30:06.3922372Z Task         : Get sources
2020-03-03T14:30:06.3922747Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T14:30:06.3923080Z Version      : 1.0.0
2020-03-03T14:30:06.3923312Z Author       : Microsoft
2020-03-03T14:30:06.3923312Z Author       : Microsoft
2020-03-03T14:30:06.3923697Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T14:30:06.3924121Z ==============================================================================
2020-03-03T14:30:06.7031876Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T14:30:06.7090178Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T14:30:06.7178594Z Cleaning up task key
2020-03-03T14:30:06.7179881Z Start cleaning up orphan processes.
2020-03-03T14:30:06.7343243Z Terminate orphan process: pid (4176) (python)
2020-03-03T14:30:06.7477876Z ##[section]Finishing: Finalize Job
