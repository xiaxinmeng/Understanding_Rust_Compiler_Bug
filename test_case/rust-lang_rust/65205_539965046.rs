plain
2019-10-09T11:20:14.4575149Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T11:20:14.4764170Z ##[command]git config gc.auto 0
2019-10-09T11:20:14.4843385Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T11:20:14.4902716Z ##[command]git config --get-all http.proxy
2019-10-09T11:20:14.5045160Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65205/merge:refs/remotes/pull/65205/merge
---
2019-10-09T11:29:06.6118172Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-09T11:30:01.7538264Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-10-09T11:30:03.8381151Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-10-09T11:30:05.2102882Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-09T11:30:05.3208066Z error: no rules expected the token `<<`
2019-10-09T11:30:05.3209343Z     |
2019-10-09T11:30:05.3209343Z     |
2019-10-09T11:30:05.3209650Z 334 | <<<<<<< HEAD
2019-10-09T11:30:05.3210026Z     | ^^ no rules expected this token in macro call
2019-10-09T11:30:05.6870824Z error[E0425]: cannot find value `E0449` in module `crate::error_codes`
2019-10-09T11:30:05.6871208Z    --> src/librustc_passes/ast_validation.rs:221:41
2019-10-09T11:30:05.6871430Z     |
2019-10-09T11:30:05.6871733Z 221 | ...                   E0449,
---
2019-10-09T11:30:05.6904810Z 
2019-10-09T11:30:05.6936098Z error[E0425]: cannot find value `E0379` in module `crate::error_codes`
2019-10-09T11:30:05.6936431Z    --> src/librustc_passes/ast_validation.rs:253:60
2019-10-09T11:30:05.6936649Z     |
2019-10-09T11:30:05.6936974Z 253 |             struct_span_err!(self.session, constness.span, E0379,
2019-10-09T11:30:05.6937345Z     |                                                            ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.6969287Z error[E0425]: cannot find value `E0472` in module `crate::error_codes`
2019-10-09T11:30:05.6969703Z    --> src/librustc_passes/ast_validation.rs:450:52
2019-10-09T11:30:05.6969966Z     |
2019-10-09T11:30:05.6969966Z     |
2019-10-09T11:30:05.6970309Z 450 |                 span_err!(self.session, expr.span, E0472, "asm! is unsupported on this target");
2019-10-09T11:30:05.6970950Z     |                                                    ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7006736Z error[E0425]: cannot find value `E0561` in module `crate::error_codes`
2019-10-09T11:30:05.7007326Z    --> src/librustc_passes/ast_validation.rs:463:58
2019-10-09T11:30:05.7007606Z     |
2019-10-09T11:30:05.7007893Z 463 |                     struct_span_err!(self.session, span, E0561,
---
2019-10-09T11:30:05.7306627Z 
2019-10-09T11:30:05.7306957Z error[E0425]: cannot find value `E0198` in module `crate::error_codes`
2019-10-09T11:30:05.7307225Z    --> src/librustc_passes/ast_validation.rs:550:56
2019-10-09T11:30:05.7307451Z     |
2019-10-09T11:30:05.7307950Z 550 |                     span_err!(self.session, item.span, E0198, "negative impls cannot be unsafe");
2019-10-09T11:30:05.7308301Z     |                                                        ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7309388Z error[E0425]: cannot find value `E0197` in module `crate::error_codes`
2019-10-09T11:30:05.7309665Z    --> src/librustc_passes/ast_validation.rs:564:56
2019-10-09T11:30:05.7309878Z     |
2019-10-09T11:30:05.7309878Z     |
2019-10-09T11:30:05.7310239Z 564 |                     span_err!(self.session, item.span, E0197, "inherent impls cannot be unsafe");
2019-10-09T11:30:05.7310780Z     |                                                        ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7311200Z error[E0425]: cannot find value `E0567` in module `crate::error_codes`
2019-10-09T11:30:05.7311494Z    --> src/librustc_passes/ast_validation.rs:614:67
2019-10-09T11:30:05.7311743Z     |
2019-10-09T11:30:05.7312103Z 614 |                         struct_span_err!(self.session, item.span, E0567,
---
2019-10-09T11:30:05.7323018Z 
2019-10-09T11:30:05.7323292Z error[E0425]: cannot find value `E0267` in module `crate::error_codes`
2019-10-09T11:30:05.7323534Z    --> src/librustc_passes/loops.rs:174:47
2019-10-09T11:30:05.7323739Z     |
2019-10-09T11:30:05.7324064Z 174 |             struct_span_err!(self.sess, span, E0267, "`{}` inside of {} {}", name, article, ty)
2019-10-09T11:30:05.7324397Z     |                                               ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7386033Z error[E0425]: cannot find value `E0268` in module `crate::error_codes`
2019-10-09T11:30:05.7386353Z    --> src/librustc_passes/loops.rs:185:51
2019-10-09T11:30:05.7386574Z     |
2019-10-09T11:30:05.7386928Z 185 |                 struct_span_err!(self.sess, span, E0268, "`{}` outside of a loop", name)
---
2019-10-09T11:30:05.7417535Z 
2019-10-09T11:30:05.7446904Z error[E0425]: cannot find value `E0136` in module `crate::error_codes`
2019-10-09T11:30:05.7448014Z    --> src/librustc_passes/entry.rs:111:52
2019-10-09T11:30:05.7448479Z     |
2019-10-09T11:30:05.7449509Z 111 |                 span_err!(ctxt.session, item.span, E0136,
2019-10-09T11:30:05.7451138Z     |                                                    ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7493674Z error[E0425]: cannot find value `E0137` in module `crate::error_codes`
2019-10-09T11:30:05.7494187Z    --> src/librustc_passes/entry.rs:122:59
2019-10-09T11:30:05.7494561Z     |
2019-10-09T11:30:05.7494561Z     |
2019-10-09T11:30:05.7495015Z 122 |                 struct_span_err!(ctxt.session, item.span, E0137,
2019-10-09T11:30:05.7495494Z     |                                                           ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7579344Z error[E0425]: cannot find value `E0138` in module `crate::error_codes`
2019-10-09T11:30:05.7618098Z    --> src/librustc_passes/entry.rs:133:59
2019-10-09T11:30:05.7618375Z     |
2019-10-09T11:30:05.7618375Z     |
2019-10-09T11:30:05.7619112Z 133 |                 struct_span_err!(ctxt.session, item.span, E0138, "multiple `start` functions")
2019-10-09T11:30:05.7619574Z     |                                                           ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7619926Z error[E0425]: cannot find value `E0601` in module `crate::error_codes`
2019-10-09T11:30:05.7620183Z    --> src/librustc_passes/entry.rs:158:41
2019-10-09T11:30:05.7620392Z     |
2019-10-09T11:30:05.7620392Z     |
2019-10-09T11:30:05.7620691Z 158 |     let mut err = struct_err!(tcx.sess, E0601,
2019-10-09T11:30:05.7621052Z     |                                         ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7621377Z error[E0425]: cannot find value `E0591` in module `crate::error_codes`
2019-10-09T11:30:05.7621644Z   --> src/librustc_passes/intrinsicck.rs:88:59
2019-10-09T11:30:05.7621858Z    |
2019-10-09T11:30:05.7622170Z 88 |                     struct_span_err!(self.tcx.sess, span, E0591,
2019-10-09T11:30:05.7622170Z 88 |                     struct_span_err!(self.tcx.sess, span, E0591,
2019-10-09T11:30:05.7622532Z    |                                                           ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:05.7622840Z 
2019-10-09T11:30:05.7623150Z error[E0425]: cannot find value `E0512` in module `crate::error_codes`
2019-10-09T11:30:05.7623437Z    --> src/librustc_passes/intrinsicck.rs:119:61
2019-10-09T11:30:05.7623645Z     |
2019-10-09T11:30:05.7623961Z 119 |         let mut err = struct_span_err!(self.tcx.sess, span, E0512,
2019-10-09T11:30:05.7624427Z     |                                                             ^^^^^ not found in `crate::error_codes`
2019-10-09T11:30:06.3764159Z error: aborting due to 28 previous errors
2019-10-09T11:30:06.3768198Z 
2019-10-09T11:30:06.3779983Z For more information about this error, try `rustc --explain E0425`.
2019-10-09T11:30:06.3945900Z error: could not compile `rustc_passes`.
---
2019-10-09T11:30:10.7812972Z == clock drift check ==
2019-10-09T11:30:10.7832383Z   local time: Wed Oct  9 11:30:10 UTC 2019
2019-10-09T11:30:10.9353975Z   network time: Wed, 09 Oct 2019 11:30:10 GMT
2019-10-09T11:30:10.9356889Z == end clock drift check ==
2019-10-09T11:30:12.1275240Z ##[error]Bash exited with code '1'.
2019-10-09T11:30:12.1322402Z ##[section]Starting: Checkout
2019-10-09T11:30:12.1324476Z ==============================================================================
2019-10-09T11:30:12.1324529Z Task         : Get sources
2019-10-09T11:30:12.1324590Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
