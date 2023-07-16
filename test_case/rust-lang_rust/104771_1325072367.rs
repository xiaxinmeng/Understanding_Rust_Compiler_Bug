plain
---- [ui] src/test/ui/rfc-2497-if-let-chains/issue-99938.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/issue-99938.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/issue-99938" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/issue-99938/auxiliary" "-Zvalidate-mir" "-C" "opt-level=3"
stdout: none
--- stderr -------------------------------
error[E0658]: `let` expressions in this position are unstable
   |
   |
LL |         if let Some(first) = inner.next()
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = help: add `#![feature(let_chains)]` to the crate attributes to enable


error[E0658]: `let` expressions in this position are unstable
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/issue-99938.rs:15:16
   |
LL |             && let Some(second) = inner.next()
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = help: add `#![feature(let_chains)]` to the crate attributes to enable


error[E0658]: `let` expressions in this position are unstable
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/issue-99938.rs:16:16
   |
LL |             && let Some(third) = inner.next()
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = help: add `#![feature(let_chains)]` to the crate attributes to enable

