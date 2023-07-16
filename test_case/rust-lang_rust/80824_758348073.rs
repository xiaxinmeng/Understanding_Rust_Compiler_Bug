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
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0432]: unresolved import `crate::boxed::WriteCloneIntoRaw`
    |
    |
268 | use crate::boxed::WriteCloneIntoRaw;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `WriteCloneIntoRaw` in `boxed`

error[E0432]: unresolved import `crate::boxed::WriteCloneIntoRaw`
   |
   |
27 | use crate::boxed::{Box, WriteCloneIntoRaw};
   |                         ^^^^^^^^^^^^^^^^^ no `WriteCloneIntoRaw` in `boxed`

error[E0599]: no method named `write_clone_into_raw` found for type parameter `T` in the current scope
     |
     |
1046 |                 (**this).write_clone_into_raw(data.as_mut_ptr());
     |                          ^^^^^^^^^^^^^^^^^^^^ method not found in `T`

error[E0599]: no method named `write_clone_into_raw` found for type parameter `T` in the current scope
     |
     |
1377 |                 (**this).write_clone_into_raw(data.as_mut_ptr());
     |                          ^^^^^^^^^^^^^^^^^^^^ method not found in `T`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "proc_macro" "-p" "panic_unwind" "-p" "panic_abort" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:50
