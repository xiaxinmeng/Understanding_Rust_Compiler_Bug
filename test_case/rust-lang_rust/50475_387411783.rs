plain
[00:05:54]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:02]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:42]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:01]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:05] warning: `#[must_use]` on methods is experimental (see issue #43302)
[00:08:05]     --> librustc/ty/sty.rs:1173:5
[00:08:05] 1173 |     #[must_use]
[00:08:05]      |     ^^^^^^^^^^^
[00:08:05]      |
[00:08:05]      |
[00:08:05]      = help: add #![feature(fn_must_use)] to the crate attributes to enable
[00:08:05] 
[00:08:05] warning: `#[must_use]` on methods is experimental (see issue #43302)
[00:08:05]     --> librustc/ty/sty.rs:1186:5
[00:08:05] 1186 |     #[must_use]
[00:08:05]      |     ^^^^^^^^^^^
[00:08:05]      |
[00:08:05]      |
[00:08:05]      = help: add #![feature(fn_must_use)] to the crate attributes to enable
[00:14:34]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:34]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:17:56]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:17:57]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
---
[00:47:19] ....................................................................................................
[00:47:23] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:33] ....................................................................................................
[00:47:39] ..........................F.........................................................................
[00:47:44] ....................................................................................................
[00:47:51] .........................................i.................FF..FFFFFFFFF....F.......................
[00:47:56] ........F........i..................................................................................
[00:48:09] ....................................................................................................
[00:48:15] ................i....................................................................
[00:48:15] failures:
[00:48:15] 
[00:48:15] 
[00:48:15] ---- [ui] ui/issue-26638.rs stdout ----
[00:48:15]  diff of stderr:
[00:48:15] 
[00:48:15] 7    = help: this function's return type contains a borrowed value, but the signature does not say which one of `iter`'s 2 lifetimes it is borrowed from
[00:48:15] 9 error[E0106]: missing lifetime specifier
[00:48:15] -   --> $DIR/issue-26638.rs:14:40
[00:48:15] -    |
[00:48:15] -    |
[00:48:15] - LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
[00:48:15] -    |                                        ^ expected lifetime parameter
[00:48:15] -    |
[00:48:15] -    = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
[00:48:15] -    = help: consider giving it an explicit bounded or 'static lifetime
[00:48:15] - error[E0106]: missing lifetime specifier
[00:48:15] 19   --> $DIR/issue-26638.rs:17:22
[00:48:15] 20    |
[00:48:15] 20    |
[00:48:15] 21 LL | fn parse_type_3() -> &str { unimplemented!() }
[00:48:15] 
[00:48:15] 24    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
[00:48:15] 25    = help: consider giving it a 'static lifetime
[00:48:15] - error: aborting due to 3 previous errors
[00:48:15] + error: aborting due to 2 previous errors
[00:48:15] 28 
[00:48:15] 29 For more information about this error, try `rustc --explain E0106`.
---
[00:48:15] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-26638.rs'
[00:48:15] 
[00:48:15] error: 1 errors occurred comparing output.
[00:48:15] status: exit code: 101
[00:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-26638.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-26638.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-26638.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] stderr:
[00:48:15] stderr:
[00:48:15] ------------------------------------------
[00:48:15] {"message":"missing lifetime specifier","code":{"code":"E0106","explanation":"\nThis error indicates that a lifetime is missing from a type. If it is an error\ninside a function signature, the problem may be with failing to adhere to the\nlifetime elision rules (see below).\n\nHere are some simple examples of where you'll run into this error:\n\n