plain
2020-01-08T03:06:20.1881679Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T03:06:20.1894496Z ##[command]git config gc.auto 0
2020-01-08T03:06:20.1903083Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T03:06:20.1906690Z ##[command]git config --get-all http.proxy
2020-01-08T03:06:20.1911477Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67516/merge:refs/remotes/pull/67516/merge
---
2020-01-08T03:14:14.3420493Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-01-08T03:14:14.5259092Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-01-08T03:14:17.2811536Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-01-08T03:14:17.5313734Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2020-01-08T03:14:17.7336898Z error[E0252]: the name `CrateNum` is defined multiple times
2020-01-08T03:14:17.7337892Z   --> src/librustc_codegen_ssa/back/link.rs:21:61
2020-01-08T03:14:17.7339203Z 12 | use rustc_hir::def_id::CrateNum;
2020-01-08T03:14:17.7339742Z    |     --------------------------- previous import of the type `CrateNum` here
2020-01-08T03:14:17.7340161Z ...
2020-01-08T03:14:17.7340161Z ...
2020-01-08T03:14:17.7340729Z 21 |     looks_like_rust_object_file, CodegenResults, CrateInfo, CrateNum, METADATA_FILENAME,
2020-01-08T03:14:17.7341285Z    |                                                             ^^^^^^^^ `CrateNum` reimported here
2020-01-08T03:14:17.7343092Z    = note: `CrateNum` must be defined only once in the type namespace of this module
2020-01-08T03:14:17.7343715Z help: you can use `as` to change the binding name of the import
2020-01-08T03:14:17.7344229Z    |
2020-01-08T03:14:17.7344229Z    |
2020-01-08T03:14:17.7345474Z 21 |     looks_like_rust_object_file, CodegenResults, CrateInfo, CrateNum as OtherCrateNum, METADATA_FILENAME,
2020-01-08T03:14:17.7351467Z 
2020-01-08T03:14:17.7817365Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-01-08T03:14:17.8962590Z error: unused import: `CrateNum`
2020-01-08T03:14:17.8962590Z error: unused import: `CrateNum`
2020-01-08T03:14:17.8963739Z   --> src/librustc_codegen_ssa/back/link.rs:21:61
2020-01-08T03:14:17.8964169Z    |
2020-01-08T03:14:17.8964743Z 21 |     looks_like_rust_object_file, CodegenResults, CrateInfo, CrateNum, METADATA_FILENAME,
2020-01-08T03:14:17.8965667Z    |
2020-01-08T03:14:17.8966141Z    = note: `-D unused-imports` implied by `-D warnings`
2020-01-08T03:14:17.8966314Z 
2020-01-08T03:14:18.5320233Z error[E0308]: mismatched types
2020-01-08T03:14:18.5320233Z error[E0308]: mismatched types
2020-01-08T03:14:18.5320561Z    --> src/librustc_codegen_ssa/back/link.rs:269:11
2020-01-08T03:14:18.5320753Z     |
2020-01-08T03:14:18.5320998Z 269 |         f(cnum, &path);
2020-01-08T03:14:18.5321290Z     |           ^^^^ expected enum `rustc_hir::def_id::CrateNum`, found struct `CrateNum`
2020-01-08T03:14:18.6749183Z error[E0308]: mismatched types
2020-01-08T03:14:18.6749183Z error[E0308]: mismatched types
2020-01-08T03:14:18.6749941Z    --> src/librustc_codegen_ssa/back/link.rs:424:59
2020-01-08T03:14:18.6750356Z     |
2020-01-08T03:14:18.6750983Z 424 |         let name = &codegen_results.crate_info.crate_name[&cnum];
2020-01-08T03:14:18.6751768Z     |                                                           ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:18.6752757Z     = note: expected reference `&CrateNum`
2020-01-08T03:14:18.6753817Z                found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:18.6753891Z 
2020-01-08T03:14:18.7999784Z error[E0308]: mismatched types
2020-01-08T03:14:18.7999784Z error[E0308]: mismatched types
2020-01-08T03:14:18.8000081Z    --> src/librustc_codegen_ssa/back/link.rs:425:72
2020-01-08T03:14:18.8000301Z     |
2020-01-08T03:14:19.3185917Z 425 |         let native_libs = &codegen_results.crate_info.native_libraries[&cnum];
2020-01-08T03:14:19.3186934Z     |                                                                        ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3187399Z     = note: expected reference `&CrateNum`
2020-01-08T03:14:19.3187803Z                found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3187836Z 
2020-01-08T03:14:19.3188047Z error[E0308]: mismatched types
2020-01-08T03:14:19.3188047Z error[E0308]: mismatched types
2020-01-08T03:14:19.3188276Z    --> src/librustc_codegen_ssa/back/link.rs:453:76
2020-01-08T03:14:19.3188583Z     |
2020-01-08T03:14:19.3188902Z 453 |         all_native_libs.extend(codegen_results.crate_info.native_libraries[&cnum].iter().cloned());
2020-01-08T03:14:19.3189287Z     |                                                                            ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3189717Z     = note: expected reference `&CrateNum`
2020-01-08T03:14:19.3190133Z                found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3190165Z 
2020-01-08T03:14:19.3190562Z error[E0308]: mismatched types
2020-01-08T03:14:19.3190562Z error[E0308]: mismatched types
2020-01-08T03:14:19.3190792Z    --> src/librustc_codegen_ssa/back/link.rs:753:44
2020-01-08T03:14:19.3190996Z     |
2020-01-08T03:14:19.3191270Z 753 |         && (info.compiler_builtins == Some(cnum) || info.is_no_builtins.contains(&cnum))
2020-01-08T03:14:19.3191625Z     |                                            ^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3191860Z error[E0308]: mismatched types
2020-01-08T03:14:19.3191860Z error[E0308]: mismatched types
2020-01-08T03:14:19.3192078Z    --> src/librustc_codegen_ssa/back/link.rs:753:82
2020-01-08T03:14:19.3192270Z     |
2020-01-08T03:14:19.3192543Z 753 |         && (info.compiler_builtins == Some(cnum) || info.is_no_builtins.contains(&cnum))
2020-01-08T03:14:19.3192922Z     |                                                                                  ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3193569Z     = note: expected reference `&CrateNum`
2020-01-08T03:14:19.3194003Z                found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:19.3194040Z 
2020-01-08T03:14:19.3628846Z error[E0308]: mismatched types
2020-01-08T03:14:19.3628846Z error[E0308]: mismatched types
2020-01-08T03:14:19.3629313Z     --> src/librustc_codegen_ssa/back/link.rs:1416:87
2020-01-08T03:14:19.3629503Z      |
2020-01-08T03:14:19.3629790Z 1416 |                 add_static_crate::<B>(cmd, sess, codegen_results, tmpdir, crate_type, cnum);
2020-01-08T03:14:19.3630164Z      |                                                                                       ^^^^ expected enum `rustc_hir::def_id::CrateNum`, found struct `CrateNum`
2020-01-08T03:14:19.4725000Z error[E0308]: mismatched types
2020-01-08T03:14:19.4725000Z error[E0308]: mismatched types
2020-01-08T03:14:19.4725358Z     --> src/librustc_codegen_ssa/back/link.rs:1422:81
2020-01-08T03:14:19.4725581Z      |
2020-01-08T03:14:19.4725933Z 1422 |                 link_sanitizer_runtime::<B>(cmd, sess, codegen_results, tmpdir, cnum);
2020-01-08T03:14:19.4726714Z      |                                                                                 ^^^^ expected enum `rustc_hir::def_id::CrateNum`, found struct `CrateNum`
2020-01-08T03:14:19.5854495Z error[E0308]: mismatched types
2020-01-08T03:14:19.5854495Z error[E0308]: mismatched types
2020-01-08T03:14:19.5854829Z     --> src/librustc_codegen_ssa/back/link.rs:1432:87
2020-01-08T03:14:19.5855075Z      |
2020-01-08T03:14:19.5855919Z 1432 |                 add_static_crate::<B>(cmd, sess, codegen_results, tmpdir, crate_type, cnum);
2020-01-08T03:14:19.5856384Z      |                                                                                       ^^^^ expected enum `rustc_hir::def_id::CrateNum`, found struct `CrateNum`
2020-01-08T03:14:19.6990411Z error[E0308]: mismatched types
2020-01-08T03:14:19.6990411Z error[E0308]: mismatched types
2020-01-08T03:14:19.6990850Z     --> src/librustc_codegen_ssa/back/link.rs:1448:79
2020-01-08T03:14:19.6991063Z      |
2020-01-08T03:14:19.6991325Z 1448 |         add_static_crate::<B>(cmd, sess, codegen_results, tmpdir, crate_type, cnum);
2020-01-08T03:14:19.6991671Z      |                                                                               ^^^^ expected enum `rustc_hir::def_id::CrateNum`, found struct `CrateNum`
2020-01-08T03:14:20.3228046Z error[E0308]: mismatched types
2020-01-08T03:14:20.3228046Z error[E0308]: mismatched types
2020-01-08T03:14:20.3228333Z     --> src/librustc_codegen_ssa/back/link.rs:1471:65
2020-01-08T03:14:20.3228558Z      |
2020-01-08T03:14:20.3229184Z 1471 |         let src = &codegen_results.crate_info.used_crate_source[&cnum];
2020-01-08T03:14:20.3229534Z      |                                                                 ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3229963Z      = note: expected reference `&CrateNum`
2020-01-08T03:14:20.3230196Z                 found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3230226Z 
2020-01-08T03:14:20.3230410Z error[E0308]: mismatched types
2020-01-08T03:14:20.3230410Z error[E0308]: mismatched types
2020-01-08T03:14:20.3230638Z     --> src/librustc_codegen_ssa/back/link.rs:1541:65
2020-01-08T03:14:20.3230805Z      |
2020-01-08T03:14:20.3231062Z 1541 |         let src = &codegen_results.crate_info.used_crate_source[&cnum];
2020-01-08T03:14:20.3231683Z      |                                                                 ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3232294Z      = note: expected reference `&CrateNum`
2020-01-08T03:14:20.3232510Z                 found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3232540Z 
2020-01-08T03:14:20.3232746Z error[E0308]: mismatched types
2020-01-08T03:14:20.3232746Z error[E0308]: mismatched types
2020-01-08T03:14:20.3232962Z     --> src/librustc_codegen_ssa/back/link.rs:1547:72
2020-01-08T03:14:20.3233370Z      |
2020-01-08T03:14:20.3233838Z 1547 |         let native_libs = &codegen_results.crate_info.native_libraries[&cnum];
2020-01-08T03:14:20.3234320Z      |                                                                        ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3234861Z      = note: expected reference `&CrateNum`
2020-01-08T03:14:20.3235131Z                 found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3235185Z 
2020-01-08T03:14:20.3235425Z error[E0308]: mismatched types
2020-01-08T03:14:20.3235425Z error[E0308]: mismatched types
2020-01-08T03:14:20.3235693Z     --> src/librustc_codegen_ssa/back/link.rs:1594:80
2020-01-08T03:14:20.3235935Z      |
2020-01-08T03:14:20.3236275Z 1594 |                         || !codegen_results.crate_info.is_no_builtins.contains(&cnum));
2020-01-08T03:14:20.3237020Z      |                                                                                ^^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3237453Z      = note: expected reference `&CrateNum`
2020-01-08T03:14:20.3237686Z                 found reference `&rustc_hir::def_id::CrateNum`
2020-01-08T03:14:20.3237714Z 
2020-01-08T03:14:20.3237901Z error[E0308]: mismatched types
2020-01-08T03:14:20.3237901Z error[E0308]: mismatched types
2020-01-08T03:14:20.3238140Z     --> src/librustc_codegen_ssa/back/link.rs:1616:73
2020-01-08T03:14:20.3238314Z      |
2020-01-08T03:14:20.3238580Z 1616 |                 && codegen_results.crate_info.compiler_builtins != Some(cnum)
2020-01-08T03:14:20.3239752Z      |                                                                         ^^^^ expected struct `CrateNum`, found enum `rustc_hir::def_id::CrateNum`
2020-01-08T03:14:21.4975778Z error: aborting due to 17 previous errors
2020-01-08T03:14:21.4975930Z 
2020-01-08T03:14:21.4976249Z Some errors have detailed explanations: E0252, E0308.
2020-01-08T03:14:21.4976511Z For more information about an error, try `rustc --explain E0252`.
2020-01-08T03:14:21.4976511Z For more information about an error, try `rustc --explain E0252`.
2020-01-08T03:14:21.5133939Z error: could not compile `rustc_codegen_ssa`.
2020-01-08T03:14:21.5134026Z 
2020-01-08T03:14:21.5134294Z To learn more, run the command again with --verbose.
2020-01-08T03:14:21.5157485Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-08T03:14:21.5166188Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-08T03:14:21.5166265Z Build completed unsuccessfully in 0:05:46
2020-01-08T03:14:21.5214571Z == clock drift check ==
2020-01-08T03:14:21.5224285Z   local time: Wed Jan  8 03:14:21 UTC 2020
2020-01-08T03:14:21.5224285Z   local time: Wed Jan  8 03:14:21 UTC 2020
2020-01-08T03:14:21.8074821Z   network time: Wed, 08 Jan 2020 03:14:21 GMT
2020-01-08T03:14:21.8080034Z == end clock drift check ==
2020-01-08T03:14:22.2898641Z 
2020-01-08T03:14:22.2964648Z ##[error]Bash exited with code '1'.
2020-01-08T03:14:22.2991206Z ##[section]Starting: Checkout
2020-01-08T03:14:22.2992785Z ==============================================================================
2020-01-08T03:14:22.2992832Z Task         : Get sources
2020-01-08T03:14:22.2992870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
