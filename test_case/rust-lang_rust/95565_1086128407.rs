plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0609]: no field `borrowck` on type `DebuggingOptions`
    |
    |
647 |     untracked!(borrowck, String::from("other"));
    |
    |
    = note: available fields are: `allow_features`, `always_encode_mir`, `assume_incomplete_release`, `asm_comments`, `assert_incr_state` ... and 150 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_interface` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
