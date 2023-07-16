plain
2019-12-23T14:40:46.4535264Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T14:40:46.4757550Z ##[command]git config gc.auto 0
2019-12-23T14:40:46.4848378Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T14:40:46.4915353Z ##[command]git config --get-all http.proxy
2019-12-23T14:40:47.0178189Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67133/merge:refs/remotes/pull/67133/merge
---
2019-12-23T14:48:10.8136314Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-23T14:48:11.3365723Z error[E0252]: the name `sign_extend` is defined multiple times
2019-12-23T14:48:11.3366148Z  --> src/librustc/ty/print/pretty.rs:8:29
2019-12-23T14:48:11.3366539Z   |
2019-12-23T14:48:11.3366947Z 7 | use crate::mir::interpret::{sign_extend, truncate, AllocId, ConstValue, Pointer, Scalar};
2019-12-23T14:48:11.3367300Z   |                             ----------- previous import of the value `sign_extend` here
2019-12-23T14:48:11.3367768Z 8 | use crate::mir::interpret::{sign_extend, truncate, ConstValue, Scalar};
2019-12-23T14:48:11.3368385Z   |                             |
2019-12-23T14:48:11.3368385Z   |                             |
2019-12-23T14:48:11.3368675Z   |                             `sign_extend` reimported here
2019-12-23T14:48:11.3368942Z   |                             help: remove unnecessary import
2019-12-23T14:48:11.3369309Z   |
2019-12-23T14:48:11.3369608Z   = note: `sign_extend` must be defined only once in the value namespace of this module
2019-12-23T14:48:11.3526658Z error[E0252]: the name `truncate` is defined multiple times
2019-12-23T14:48:11.3526959Z  --> src/librustc/ty/print/pretty.rs:8:42
2019-12-23T14:48:11.3527211Z   |
2019-12-23T14:48:11.3527211Z   |
2019-12-23T14:48:11.3527532Z 7 | use crate::mir::interpret::{sign_extend, truncate, AllocId, ConstValue, Pointer, Scalar};
2019-12-23T14:48:11.3527861Z   |                                          -------- previous import of the value `truncate` here
2019-12-23T14:48:11.3528170Z 8 | use crate::mir::interpret::{sign_extend, truncate, ConstValue, Scalar};
2019-12-23T14:48:11.3528874Z   |                                          |
2019-12-23T14:48:11.3528874Z   |                                          |
2019-12-23T14:48:11.3529144Z   |                                          `truncate` reimported here
2019-12-23T14:48:11.3529583Z   |                                          help: remove unnecessary import
2019-12-23T14:48:11.3530224Z   = note: `truncate` must be defined only once in the value namespace of this module
2019-12-23T14:48:11.3530259Z 
2019-12-23T14:48:12.9333438Z error[E0252]: the name `ConstValue` is defined multiple times
2019-12-23T14:48:12.9333903Z  --> src/librustc/ty/print/pretty.rs:8:52
2019-12-23T14:48:12.9333903Z  --> src/librustc/ty/print/pretty.rs:8:52
2019-12-23T14:48:12.9334213Z   |
2019-12-23T14:48:12.9334550Z 7 | use crate::mir::interpret::{sign_extend, truncate, AllocId, ConstValue, Pointer, Scalar};
2019-12-23T14:48:12.9335312Z   |                                                             ---------- previous import of the type `ConstValue` here
2019-12-23T14:48:12.9335819Z 8 | use crate::mir::interpret::{sign_extend, truncate, ConstValue, Scalar};
2019-12-23T14:48:12.9336657Z   |                                                    |
2019-12-23T14:48:12.9336919Z   |                                                    `ConstValue` reimported here
2019-12-23T14:48:12.9337217Z   |                                                    help: remove unnecessary import
2019-12-23T14:48:12.9337402Z   |
2019-12-23T14:48:12.9337402Z   |
2019-12-23T14:48:12.9337681Z   = note: `ConstValue` must be defined only once in the type namespace of this module
2019-12-23T14:48:12.9341771Z 
2019-12-23T14:48:12.9448615Z error[E0252]: the name `Scalar` is defined multiple times
2019-12-23T14:48:12.9448921Z  --> src/librustc/ty/print/pretty.rs:8:64
2019-12-23T14:48:12.9449120Z   |
2019-12-23T14:48:12.9449404Z 7 | use crate::mir::interpret::{sign_extend, truncate, AllocId, ConstValue, Pointer, Scalar};
2019-12-23T14:48:12.9449772Z   |                                                                                  ------ previous import of the type `Scalar` here
2019-12-23T14:48:12.9450072Z 8 | use crate::mir::interpret::{sign_extend, truncate, ConstValue, Scalar};
2019-12-23T14:48:12.9450425Z   |                                                                ^^^^^^ `Scalar` reimported here
2019-12-23T14:48:12.9450623Z   |
2019-12-23T14:48:12.9450885Z   = note: `Scalar` must be defined only once in the type namespace of this module
2019-12-23T14:48:13.3381869Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-23T14:48:14.1449421Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.1449778Z    --> src/librustc/ty/print/pretty.rs:765:21
2019-12-23T14:48:14.1449974Z     |
2019-12-23T14:48:14.1449974Z     |
2019-12-23T14:48:14.1450305Z 765 |                     GenericArgKind::Lifetime(r) => *r != ty::ReErased,
2019-12-23T14:48:14.1450679Z 
2019-12-23T14:48:14.1474900Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.1475209Z    --> src/librustc/ty/print/pretty.rs:769:21
2019-12-23T14:48:14.1475472Z     |
2019-12-23T14:48:14.1475472Z     |
2019-12-23T14:48:14.1475793Z 769 |                     GenericArgKind::Lifetime(_) => print_regions,
2019-12-23T14:48:14.1476355Z     |                     ^^^^^^^^^^^^^^ use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.1476660Z 
2019-12-23T14:48:14.1656486Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.1657033Z     --> src/librustc/ty/print/pretty.rs:1350:13
2019-12-23T14:48:14.1657278Z      |
2019-12-23T14:48:14.1657726Z 1350 |             GenericArgKind::Lifetime(r) => *r != ty::ReErased,
2019-12-23T14:48:14.1658113Z 
2019-12-23T14:48:14.1689428Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.1689727Z     --> src/librustc/ty/print/pretty.rs:1354:13
2019-12-23T14:48:14.1690156Z      |
2019-12-23T14:48:14.1690156Z      |
2019-12-23T14:48:14.1690439Z 1354 |             GenericArgKind::Lifetime(_) => print_regions,
2019-12-23T14:48:14.1690765Z      |             ^^^^^^^^^^^^^^ use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.1690831Z 
2019-12-23T14:48:14.2193882Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.2194272Z     --> src/librustc/ty/print/pretty.rs:1937:13
2019-12-23T14:48:14.2194558Z      |
2019-12-23T14:48:14.2194879Z 1937 |             GenericArgKind::Lifetime(lt) => p!(print(lt)),
2019-12-23T14:48:14.2195377Z 
2019-12-23T14:48:14.2243901Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.2246610Z     --> src/librustc/ty/print/pretty.rs:1938:13
2019-12-23T14:48:14.2247025Z      |
2019-12-23T14:48:14.2247025Z      |
2019-12-23T14:48:14.2247303Z 1938 |             GenericArgKind::Type(ty) => p!(print(ty)),
2019-12-23T14:48:14.2247852Z 
2019-12-23T14:48:14.2259407Z error[E0433]: failed to resolve: use of undeclared type or module `GenericArgKind`
2019-12-23T14:48:14.2259706Z     --> src/librustc/ty/print/pretty.rs:1939:13
2019-12-23T14:48:14.2259964Z      |
2019-12-23T14:48:14.2259964Z      |
2019-12-23T14:48:14.2260235Z 1939 |             GenericArgKind::Const(ct) => p!(print(ct)),
2019-12-23T14:48:14.2264744Z 
2019-12-23T14:48:14.6545184Z error[E0404]: expected trait, found derive macro `TypeFoldable`
2019-12-23T14:48:14.6545641Z    --> src/librustc/ty/print/pretty.rs:195:68
2019-12-23T14:48:14.6545976Z     |
2019-12-23T14:48:14.6545976Z     |
2019-12-23T14:48:14.6547415Z 195 |         T: Print<'tcx, Self, Output = Self, Error = Self::Error> + TypeFoldable<'tcx>,
2019-12-23T14:48:14.6548129Z     |
2019-12-23T14:48:14.6548406Z help: possible better candidate is found in another module, you can import it into scope
2019-12-23T14:48:14.6548595Z     |
2019-12-23T14:48:14.6548829Z 1   | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.6548829Z 1   | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.6549034Z     |
2019-12-23T14:48:14.6577906Z 
2019-12-23T14:48:14.6937667Z error[E0422]: cannot find struct, variant or union type `ParamConst` in this scope
2019-12-23T14:48:14.6938313Z    --> src/librustc/ty/print/pretty.rs:874:35
2019-12-23T14:48:14.6939316Z     |
2019-12-23T14:48:14.6939804Z 874 |             (ty::ConstKind::Param(ParamConst { name, .. }), _) => p!(write("{}", name)),
2019-12-23T14:48:14.6940830Z     |
2019-12-23T14:48:14.6941628Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T14:48:14.6941995Z     |
2019-12-23T14:48:14.6941995Z     |
2019-12-23T14:48:14.6942549Z 1   | use crate::ty::sty::ParamConst;
2019-12-23T14:48:14.6943771Z 
2019-12-23T14:48:14.7354375Z error[E0404]: expected trait, found derive macro `TypeFoldable`
2019-12-23T14:48:14.7355281Z     --> src/librustc/ty/print/pretty.rs:1388:68
2019-12-23T14:48:14.7355742Z      |
2019-12-23T14:48:14.7355742Z      |
2019-12-23T14:48:14.7356307Z 1388 |         T: Print<'tcx, Self, Output = Self, Error = Self::Error> + TypeFoldable<'tcx>,
2019-12-23T14:48:14.7357399Z      |
2019-12-23T14:48:14.7357888Z help: possible better candidate is found in another module, you can import it into scope
2019-12-23T14:48:14.7358260Z      |
2019-12-23T14:48:14.7358873Z 1    | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.7358873Z 1    | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.7359229Z      |
2019-12-23T14:48:14.7359432Z 
2019-12-23T14:48:14.7684875Z error[E0404]: expected trait, found derive macro `TypeFoldable`
2019-12-23T14:48:14.7685399Z     --> src/librustc/ty/print/pretty.rs:1588:67
2019-12-23T14:48:14.7685709Z      |
2019-12-23T14:48:14.7686056Z 1588 |         T: Print<'tcx, Self, Output = Self, Error = fmt::Error> + TypeFoldable<'tcx>,
2019-12-23T14:48:14.7686887Z      |
2019-12-23T14:48:14.7687162Z help: possible better candidate is found in another module, you can import it into scope
2019-12-23T14:48:14.7687627Z      |
2019-12-23T14:48:14.7688049Z 1    | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.7688049Z 1    | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.7688252Z      |
2019-12-23T14:48:14.7688313Z 
2019-12-23T14:48:14.7976449Z error[E0404]: expected trait, found derive macro `TypeFoldable`
2019-12-23T14:48:14.7976827Z     --> src/librustc/ty/print/pretty.rs:1654:67
2019-12-23T14:48:14.7977180Z      |
2019-12-23T14:48:14.7977494Z 1654 |         T: Print<'tcx, Self, Output = Self, Error = fmt::Error> + TypeFoldable<'tcx>,
2019-12-23T14:48:14.7978142Z      |
2019-12-23T14:48:14.7978439Z help: possible better candidate is found in another module, you can import it into scope
2019-12-23T14:48:14.7978678Z      |
2019-12-23T14:48:14.7978936Z 1    | use crate::ty::fold::TypeFoldable;
---
2019-12-23T14:48:14.8268565Z 
2019-12-23T14:48:14.8558544Z error[E0404]: expected trait, found derive macro `TypeFoldable`
2019-12-23T14:48:14.8558896Z     --> src/librustc/ty/print/pretty.rs:1690:55
2019-12-23T14:48:14.8559127Z      |
2019-12-23T14:48:14.8559531Z 1690 |     T: Print<'tcx, P, Output = P, Error = P::Error> + TypeFoldable<'tcx>,
2019-12-23T14:48:14.8560096Z      |
2019-12-23T14:48:14.8560349Z help: possible better candidate is found in another module, you can import it into scope
2019-12-23T14:48:14.8560564Z      |
2019-12-23T14:48:14.8560804Z 1    | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.8560804Z 1    | use crate::ty::fold::TypeFoldable;
2019-12-23T14:48:14.8560988Z      |
2019-12-23T14:48:14.8561042Z 
2019-12-23T14:48:15.1045592Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-23T14:48:15.9175543Z error: unused imports: `ConstValue`, `Scalar`, `sign_extend`, `truncate`
2019-12-23T14:48:15.9176431Z  --> src/librustc/ty/print/pretty.rs:8:29
2019-12-23T14:48:15.9177371Z   |
2019-12-23T14:48:15.9177862Z 8 | use crate::mir::interpret::{sign_extend, truncate, ConstValue, Scalar};
2019-12-23T14:48:15.9178542Z   |
2019-12-23T14:48:15.9178831Z   = note: `-D unused-imports` implied by `-D warnings`
2019-12-23T14:48:15.9178895Z 
2019-12-23T14:48:18.0159020Z error: aborting due to 19 previous errors
2019-12-23T14:48:18.0159020Z error: aborting due to 19 previous errors
2019-12-23T14:48:18.0159917Z 
2019-12-23T14:48:18.0160524Z Some errors have detailed explanations: E0252, E0404, E0422, E0433.
2019-12-23T14:48:18.0161230Z For more information about an error, try `rustc --explain E0252`.
2019-12-23T14:48:18.0481130Z error: could not compile `rustc`.
2019-12-23T14:48:18.0482076Z 
2019-12-23T14:48:18.0483040Z To learn more, run the command again with --verbose.
2019-12-23T14:48:18.0511724Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T14:48:18.0526970Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T14:48:18.0527301Z Build completed unsuccessfully in 0:04:20
2019-12-23T14:48:18.0587077Z == clock drift check ==
2019-12-23T14:48:18.0607120Z   local time: Mon Dec 23 14:48:18 UTC 2019
2019-12-23T14:48:18.0607120Z   local time: Mon Dec 23 14:48:18 UTC 2019
2019-12-23T14:48:18.3398535Z   network time: Mon, 23 Dec 2019 14:48:18 GMT
2019-12-23T14:48:18.3402172Z == end clock drift check ==
2019-12-23T14:48:19.3478655Z 
2019-12-23T14:48:19.3595428Z ##[error]Bash exited with code '1'.
2019-12-23T14:48:19.3624278Z ##[section]Starting: Checkout
2019-12-23T14:48:19.3626199Z ==============================================================================
2019-12-23T14:48:19.3626255Z Task         : Get sources
2019-12-23T14:48:19.3626304Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
