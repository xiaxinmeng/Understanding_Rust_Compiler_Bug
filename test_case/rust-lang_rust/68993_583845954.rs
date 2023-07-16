plain
2020-02-09T13:26:37.9849928Z ========================== Starting Command Output ===========================
2020-02-09T13:26:37.9852354Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c806439a-3cae-42d2-acca-ed8628dd3811.sh
2020-02-09T13:26:37.9852399Z 
2020-02-09T13:26:37.9855920Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T13:26:37.9862536Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68993/merge to s
2020-02-09T13:26:37.9864364Z Task         : Get sources
2020-02-09T13:26:37.9864394Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T13:26:37.9864467Z Version      : 1.0.0
2020-02-09T13:26:37.9864497Z Author       : Microsoft
---
2020-02-09T13:26:38.9873348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T13:26:38.9884322Z ##[command]git config gc.auto 0
2020-02-09T13:26:38.9886375Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T13:26:38.9888064Z ##[command]git config --get-all http.proxy
2020-02-09T13:26:38.9894603Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68993/merge:refs/remotes/pull/68993/merge
---
2020-02-09T13:34:12.8355423Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-02-09T13:34:15.1224454Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-02-09T13:34:16.3334160Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-02-09T13:34:17.3170466Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-02-09T13:34:20.2391133Z error[E0609]: no field `var_used_at` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2391569Z   --> src/librustc_mir/borrow_check/facts.rs:74:17
2020-02-09T13:34:20.2392099Z 74 |                 var_used_at,
2020-02-09T13:34:20.2392790Z    |                 ^^^^^^^^^^^ help: a field with a similar name exists: `var_used`
2020-02-09T13:34:20.2392825Z 
2020-02-09T13:34:20.2392825Z 
2020-02-09T13:34:20.2431036Z error[E0609]: no field `var_defined_at` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2431454Z   --> src/librustc_mir/borrow_check/facts.rs:75:17
2020-02-09T13:34:20.2432174Z 75 |                 var_defined_at,
2020-02-09T13:34:20.2432718Z    |                 ^^^^^^^^^^^^^^ help: a field with a similar name exists: `var_defined`
2020-02-09T13:34:20.2432780Z 
2020-02-09T13:34:20.2432780Z 
2020-02-09T13:34:20.2479541Z error[E0609]: no field `var_dropped_at` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2479879Z   --> src/librustc_mir/borrow_check/facts.rs:76:17
2020-02-09T13:34:20.2480529Z 76 |                 var_dropped_at,
2020-02-09T13:34:20.2480890Z    |                 ^^^^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.2481175Z    |
2020-02-09T13:34:20.2481175Z    |
2020-02-09T13:34:20.2481565Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2481632Z 
2020-02-09T13:34:20.2526259Z error[E0609]: no field `use_of_var_derefs_origin` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2526538Z   --> src/librustc_mir/borrow_check/facts.rs:77:17
2020-02-09T13:34:20.2526800Z    |
2020-02-09T13:34:20.2527052Z 77 |                 use_of_var_derefs_origin,
2020-02-09T13:34:20.2527903Z    |
2020-02-09T13:34:20.2527903Z    |
2020-02-09T13:34:20.2528200Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2528239Z 
2020-02-09T13:34:20.2566564Z error[E0609]: no field `drop_of_var_derefs_origin` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2566821Z   --> src/librustc_mir/borrow_check/facts.rs:78:17
2020-02-09T13:34:20.2567027Z    |
2020-02-09T13:34:20.2567264Z 78 |                 drop_of_var_derefs_origin,
2020-02-09T13:34:20.2567737Z    |
2020-02-09T13:34:20.2567737Z    |
2020-02-09T13:34:20.2568008Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2568053Z 
2020-02-09T13:34:20.2608706Z error[E0609]: no field `child_path` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2608977Z   --> src/librustc_mir/borrow_check/facts.rs:79:17
2020-02-09T13:34:20.2609978Z 79 |                 child_path,
2020-02-09T13:34:20.2610474Z    |                 ^^^^^^^^^^ unknown field
2020-02-09T13:34:20.2610777Z    |
2020-02-09T13:34:20.2610777Z    |
2020-02-09T13:34:20.2611133Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2611300Z 
2020-02-09T13:34:20.2652264Z error[E0609]: no field `path_is_var` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2652691Z   --> src/librustc_mir/borrow_check/facts.rs:80:17
2020-02-09T13:34:20.2653096Z 80 |                 path_is_var,
2020-02-09T13:34:20.2653349Z    |                 ^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.2653534Z    |
2020-02-09T13:34:20.2653534Z    |
2020-02-09T13:34:20.2653827Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2653870Z 
2020-02-09T13:34:20.2694003Z error[E0609]: no field `path_assigned_at_base` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2694478Z   --> src/librustc_mir/borrow_check/facts.rs:81:17
2020-02-09T13:34:20.2694901Z 81 |                 path_assigned_at_base,
2020-02-09T13:34:20.2695194Z    |                 ^^^^^^^^^^^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.2695375Z    |
2020-02-09T13:34:20.2695375Z    |
2020-02-09T13:34:20.2695674Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2695732Z 
2020-02-09T13:34:20.2736781Z error[E0609]: no field `path_moved_at_base` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2737066Z   --> src/librustc_mir/borrow_check/facts.rs:82:17
2020-02-09T13:34:20.2737520Z 82 |                 path_moved_at_base,
2020-02-09T13:34:20.2737792Z    |                 ^^^^^^^^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.2738005Z    |
2020-02-09T13:34:20.2738005Z    |
2020-02-09T13:34:20.2738460Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.2738497Z 
2020-02-09T13:34:20.2784800Z error[E0609]: no field `path_accessed_at_base` on type `&polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.2785080Z   --> src/librustc_mir/borrow_check/facts.rs:83:17
2020-02-09T13:34:20.2786067Z 83 |                 path_accessed_at_base,
2020-02-09T13:34:20.2786657Z    |                 ^^^^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `path_accessed_at`
2020-02-09T13:34:20.2786706Z 
2020-02-09T13:34:20.2786706Z 
2020-02-09T13:34:20.3364443Z error[E0609]: no field `path_is_var` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3364789Z   --> src/librustc_mir/borrow_check/nll.rs:87:10
2020-02-09T13:34:20.3365343Z 87 |         .path_is_var
2020-02-09T13:34:20.3365584Z    |          ^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.3365781Z    |
2020-02-09T13:34:20.3365781Z    |
2020-02-09T13:34:20.3366050Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.3366084Z 
2020-02-09T13:34:20.3407617Z error[E0609]: no field `child_path` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3407884Z   --> src/librustc_mir/borrow_check/nll.rs:96:23
2020-02-09T13:34:20.3408066Z    |
2020-02-09T13:34:20.3408323Z 96 |             all_facts.child_path.push((child, parent));
2020-02-09T13:34:20.3409230Z    |
2020-02-09T13:34:20.3409230Z    |
2020-02-09T13:34:20.3409615Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.3409662Z 
2020-02-09T13:34:20.3458990Z error[E0609]: no field `path_assigned_at_base` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3459403Z    --> src/librustc_mir/borrow_check/nll.rs:122:30
2020-02-09T13:34:20.3459958Z 122 | ...                   .path_assigned_at_base
2020-02-09T13:34:20.3460338Z     |                        ^^^^^^^^^^^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.3460561Z     |
2020-02-09T13:34:20.3460561Z     |
2020-02-09T13:34:20.3460916Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.3460989Z 
2020-02-09T13:34:20.3489255Z error[E0609]: no field `path_assigned_at_base` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3489527Z    --> src/librustc_mir/borrow_check/nll.rs:128:31
2020-02-09T13:34:20.3489714Z     |
2020-02-09T13:34:20.3489981Z 128 |                     all_facts.path_assigned_at_base.push((init.path, location_table.mid_index(location)));
2020-02-09T13:34:20.3490443Z     |
2020-02-09T13:34:20.3490443Z     |
2020-02-09T13:34:20.3490724Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.3490758Z 
2020-02-09T13:34:20.3520293Z error[E0609]: no field `path_assigned_at_base` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3520699Z    --> src/librustc_mir/borrow_check/nll.rs:134:27
2020-02-09T13:34:20.3520925Z     |
2020-02-09T13:34:20.3521171Z 134 |                 all_facts.path_assigned_at_base.push((init.path, fn_entry_start));
2020-02-09T13:34:20.3521724Z     |
2020-02-09T13:34:20.3521724Z     |
2020-02-09T13:34:20.3521997Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.3522033Z 
2020-02-09T13:34:20.3561658Z error[E0609]: no field `path_moved_at_base` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3562045Z    --> src/librustc_mir/borrow_check/nll.rs:144:23
2020-02-09T13:34:20.3562280Z     |
2020-02-09T13:34:20.3562875Z 144 |             all_facts.path_moved_at_base.push((path, fn_entry_start));
2020-02-09T13:34:20.3563345Z     |
2020-02-09T13:34:20.3563345Z     |
2020-02-09T13:34:20.3563791Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.3563848Z 
2020-02-09T13:34:20.3591136Z error[E0609]: no field `path_moved_at_base` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.3591513Z    --> src/librustc_mir/borrow_check/nll.rs:152:10
2020-02-09T13:34:20.3592192Z 152 |         .path_moved_at_base
2020-02-09T13:34:20.3592451Z     |          ^^^^^^^^^^^^^^^^^^ unknown field
2020-02-09T13:34:20.3592628Z     |
2020-02-09T13:34:20.3592628Z     |
2020-02-09T13:34:20.3592884Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.5299391Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-02-09T13:34:20.7712520Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-02-09T13:34:20.7712520Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-02-09T13:34:20.9583913Z error[E0609]: no field `var_defined_at` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9584228Z   --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:97:40
2020-02-09T13:34:20.9584446Z    |
2020-02-09T13:34:20.9584705Z 97 |             var_defined_at: &mut facts.var_defined_at,
2020-02-09T13:34:20.9585053Z    |                                        ^^^^^^^^^^^^^^ help: a field with a similar name exists: `var_defined`
2020-02-09T13:34:20.9609749Z 
2020-02-09T13:34:20.9614703Z error[E0609]: no field `var_used_at` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9615005Z   --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:98:37
2020-02-09T13:34:20.9615359Z    |
2020-02-09T13:34:20.9615641Z 98 |             var_used_at: &mut facts.var_used_at,
2020-02-09T13:34:20.9615961Z    |                                     ^^^^^^^^^^^ help: a field with a similar name exists: `var_used`
2020-02-09T13:34:20.9633862Z 
2020-02-09T13:34:20.9638456Z error[E0609]: no field `var_dropped_at` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9639240Z   --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:99:40
2020-02-09T13:34:20.9639489Z    |
2020-02-09T13:34:20.9639819Z 99 |             var_dropped_at: &mut facts.var_dropped_at,
2020-02-09T13:34:20.9640451Z    |
2020-02-09T13:34:20.9640451Z    |
2020-02-09T13:34:20.9640809Z    = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.9664497Z 
2020-02-09T13:34:20.9669291Z error[E0609]: no field `path_accessed_at_base` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9669646Z    --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:100:47
2020-02-09T13:34:20.9669906Z     |
2020-02-09T13:34:20.9670229Z 100 |             path_accessed_at_base: &mut facts.path_accessed_at_base,
2020-02-09T13:34:20.9670854Z     |                                               ^^^^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `path_accessed_at`
2020-02-09T13:34:20.9692693Z 
2020-02-09T13:34:20.9697359Z error[E0609]: no field `var_dropped_at` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9697653Z    --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:106:15
2020-02-09T13:34:20.9697828Z     |
2020-02-09T13:34:20.9698042Z 106 |         facts.var_dropped_at.extend(
2020-02-09T13:34:20.9698468Z     |
2020-02-09T13:34:20.9698468Z     |
2020-02-09T13:34:20.9699225Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.9738950Z 
2020-02-09T13:34:20.9744174Z error[E0609]: no field `use_of_var_derefs_origin` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9744488Z    --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:116:23
2020-02-09T13:34:20.9744662Z     |
2020-02-09T13:34:20.9744898Z 116 |                 facts.use_of_var_derefs_origin.push((local, region_vid));
2020-02-09T13:34:20.9745339Z     |
2020-02-09T13:34:20.9745339Z     |
2020-02-09T13:34:20.9745768Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:20.9780257Z 
2020-02-09T13:34:20.9785542Z error[E0609]: no field `drop_of_var_derefs_origin` on type `&mut polonius_engine::facts::AllFacts<borrow_check::facts::RustcFacts>`
2020-02-09T13:34:20.9785834Z    --> src/librustc_mir/borrow_check/type_check/liveness/polonius.rs:135:19
2020-02-09T13:34:20.9786010Z     |
2020-02-09T13:34:20.9786244Z 135 |             facts.drop_of_var_derefs_origin.push((local, region_vid));
2020-02-09T13:34:20.9786697Z     |
2020-02-09T13:34:20.9786697Z     |
2020-02-09T13:34:20.9786975Z     = note: available fields are: `borrow_region`, `universal_region`, `cfg_edge`, `killed`, `outlives` ... and 13 others
2020-02-09T13:34:23.5772848Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2020-02-09T13:34:24.0733970Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-02-09T13:34:25.1025144Z error: aborting due to 24 previous errors
2020-02-09T13:34:25.1025364Z 
2020-02-09T13:34:25.1025364Z 
2020-02-09T13:34:25.1026063Z For more information about this error, try `rustc --explain E0609`.
2020-02-09T13:34:25.1026411Z error: could not compile `rustc_mir`.
2020-02-09T13:34:25.1026744Z warning: build failed, waiting for other jobs to finish...
2020-02-09T13:34:26.8821122Z error: build failed
2020-02-09T13:34:26.8839501Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-09T13:34:26.8846280Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-09T13:34:26.8846560Z Build completed unsuccessfully in 0:05:25
2020-02-09T13:34:26.8889999Z == clock drift check ==
2020-02-09T13:34:26.8906544Z   local time: Sun Feb  9 13:34:26 UTC 2020
2020-02-09T13:34:26.8906544Z   local time: Sun Feb  9 13:34:26 UTC 2020
2020-02-09T13:34:27.4238092Z   network time: Sun, 09 Feb 2020 13:34:27 GMT
2020-02-09T13:34:27.4241787Z == end clock drift check ==
2020-02-09T13:34:28.0952507Z 
2020-02-09T13:34:28.1036091Z ##[error]Bash exited with code '1'.
2020-02-09T13:34:28.1048177Z ##[section]Finishing: Run build
2020-02-09T13:34:28.1098551Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68993/merge to s
2020-02-09T13:34:28.1100574Z Task         : Get sources
2020-02-09T13:34:28.1100658Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T13:34:28.1100705Z Version      : 1.0.0
2020-02-09T13:34:28.1100746Z Author       : Microsoft
2020-02-09T13:34:28.1100746Z Author       : Microsoft
2020-02-09T13:34:28.1100809Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T13:34:28.1100859Z ==============================================================================
2020-02-09T13:34:28.4788882Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T13:34:28.4825933Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68993/merge to s
2020-02-09T13:34:28.4930190Z Cleaning up task key
2020-02-09T13:34:28.4930966Z Start cleaning up orphan processes.
2020-02-09T13:34:28.5027741Z Terminate orphan process: pid (6559) (python)
2020-02-09T13:34:28.5218682Z ##[section]Finishing: Finalize Job
