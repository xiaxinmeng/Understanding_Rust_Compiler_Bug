plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0277]: the trait bound `&&TyS<'_>: TypeFoldable<'_>` is not satisfied
    --> compiler/rustc_typeck/src/check/coercion.rs:1493:58
     |
1493 | ...                   fcx.resolve_vars_if_possible(&expected)
     |                                                    ^^^^^^^^^ the trait `TypeFoldable<'_>` is not implemented for `&&TyS<'_>`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <&'tcx TyS<'tcx> as TypeFoldable<'tcx>>
help: consider adding dereference here
     |
1493 |                             fcx.resolve_vars_if_possible(&*expected)
     |                                                          ^^^^^^^^^^
help: consider removing the leading `&`-reference
     |
1493 |                             fcx.resolve_vars_if_possible(expected)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:11:09
== clock drift check ==
  local time: Sat Oct 24 00:40:50 UTC 2020
  local time: Sat Oct 24 00:40:50 UTC 2020
  network time: Fri, 23 Oct 2020 23:31:30 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6810) (python)
