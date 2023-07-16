plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0609]: no field `incremental_ignore_spans` on type `DebuggingOptions`
    |
    |
638 |     untracked!(incremental_ignore_spans, true);
    |
    |
    = note: available fields are: `allow_features`, `always_encode_mir`, `assume_incomplete_release`, `asm_comments`, `ast_json` ... and 131 others
error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_interface`
