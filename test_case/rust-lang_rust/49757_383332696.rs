plain
[00:42:25] .................................................................................i..................
[00:42:30] .........................i..........................................................................
[00:42:34] ....................................................................................................
[00:42:38] ....................................................................................................
[00:42:42] .......................................................................F............................
[00:42:52] ....................................................................................................
[00:42:58] ....................................................................................................
[00:43:03] ....................................................................................................
[00:43:10] .........................i..........................................................................
---
[00:43:34] 
[00:43:34] ---- [ui] ui/feature-gate-doc_alias.rs stdout ----
[00:43:34]  diff of stderr:
[00:43:34] 
[00:43:34] - error[E0658]: #[doc(alias = "...")] is experimental
[00:43:34] + error[E0658]: #[doc(alias = "...")] is experimental (see issue #50146)
[00:43:34] 2   --> $DIR/feature-gate-doc_alias.rs:11:1
[00:43:34] 3    |
[00:43:34] 4 LL | #[doc(alias = "foo")] //~ ERROR: #[doc(alias = "...")] is experimental
[00:43:34] 
[00:43:34] The actual stderr differed from the expected stderr.
[00:43:34] The actual stderr differed from the expected stderr.
[00:43:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-doc_alias.stderr
[00:43:34] To update references, run this command from build directory:
[00:43:34] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'feature-gate-doc_alias.rs'
[00:43:34] error: 1 errors occurred comparing output.
[00:43:34] status: exit code: 101
[00:43:34] status: exit code: 101
[00:43:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-doc_alias.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-doc_alias.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-doc_alias.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:34] ------------------------------------------
[00:43:34] 
[00:43:34] ------------------------------------------
[00:43:34] stderr:
[00:43:34] stderr:
[00:43:34] ------------------------------------------
[00:43:34] {"message":"#[doc(alias = \"...\")] is experimental (see issue #50146)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n