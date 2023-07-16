plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
error: referenced message `assoc_item` does not exist
  --> compiler/rustc_error_messages/src/lib.rs:55:14
   |
55 |     infer => "../locales/en-US/infer.ftl",
   |
   |
   = help: you may have meant to use a variable reference (`{{${mref}}}`)

error: referenced message `lifetime_1` does not exist
   |
   |
55 |     infer => "../locales/en-US/infer.ftl",
   |
   |
   = help: you may have meant to use a variable reference (`{{${mref}}}`)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
error: could not compile `rustc_error_messages` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_error_messages` due to 2 previous errors
