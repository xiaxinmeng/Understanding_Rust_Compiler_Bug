plain
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:53:94
   |
53 |                 rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_path(path, false))
   |                                                                                              ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:362:90
    |
    |
362 |             rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_path(path, false))
    |                                                                                          ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:462:95
    |
    |
462 |                 rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_qpath(path, false))
    |                                                                                               ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/utils/inspector.rs:479:95
    |
    |
479 |                 rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_qpath(path, false))
    |                                                                                               ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/match_result_ok.rs:65:90
   |
   |
65 |             if rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_path(x, false)) == "Some";
   |                                                                                          ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/matches.rs:857:91
    |
    |
857 |             rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_qpath(path, false))
    |                                                                                           ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/matches.rs:861:91
    |
    |
861 |             rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_qpath(path, false))
    |                                                                                           ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/matches.rs:960:110
    |
    |
960 |                 let path_str = rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_qpath(path, false));
    |                                                                                                              ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/mut_reference.rs:43:104
   |
   |
43 |                         &rustc_hir_pretty::to_string(rustc_hir_pretty::NO_ANN, |s| s.print_qpath(path, false)),
   |                                                                                                        ^^^^^ expected struct `ColonsBeforeParams`, found `bool`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 9 previous errors
Build completed unsuccessfully in 0:03:14
