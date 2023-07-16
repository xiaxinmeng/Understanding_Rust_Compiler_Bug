plain
[00:49:18] ....................................................................................................
[00:49:21] ....................................................................................................
[00:49:25] ....................................................................................................
[00:49:28] ....................................................................................................
[00:49:33] ........F...........................................................................................
[00:49:37] ...................F................................................................................
[00:49:49] ....................................................................................................
[00:49:55] .....i..............................................................................i...............
[00:49:55] .....i..............................................................................i...............
[00:50:00] ..................................................................F.................................
[00:50:12] ....................................................................................................
[00:50:19] ...
[00:50:19] failures:
[00:50:19] 
[00:50:19] 
[00:50:19] ---- [ui] ui/feature-gate/issue-43106-gating-of-inline.rs stdout ----
[00:50:19] diff of stderr:
[00:50:19] 
[00:50:19] 14    | |_- not a function or closure
[00:50:19] 15 
[00:50:19] 16 error[E0518]: attribute should be applied to function or closure
[00:50:19] -    |
[00:50:19] -    |
[00:50:19] - LL |     mod inner { #![inline="2100"] }
[00:50:19] -    |     ------------^^^^^^^^^^^^^^^^^-- not a function or closure
[00:50:19] - 
[00:50:19] - error[E0518]: attribute should be applied to function or closure
[00:50:19] 24    |
[00:50:19] 24    |
[00:50:19] 25 LL |     #[inline = "2100"] struct S;
[00:50:19] 36    |
[00:50:19] 36    |
[00:50:19] 37 LL |     #[inline = "2100"] impl S { }
[00:50:19] 38    |     ^^^^^^^^^^^^^^^^^^ ---------- not a function or closure
[00:50:19] + 
[00:50:19] + error[E0518]: attribute should be applied to function or closure
[00:50:19] +   --> $DIR/issue-43106-gating-of-inline.rs:24:17
[00:50:19] +    |
[00:50:19] + LL |     mod inner { #![inline="2100"] }
[00:50:19] +    |     ------------^^^^^^^^^^^^^^^^^-- not a function or closure
[00:50:19] 40 error: aborting due to 5 previous errors
[00:50:19] 41 
[00:50:19] 
[00:50:19] 
[00:50:19] 
[00:50:19] The actual stderr differed from the expected stderr.
[00:50:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-inline/issue-43106-gating-of-inline.stderr
[00:50:19] To update references, rerun the tests and pass the `--bless` flag
[00:50:19] To only update this specific test, also pass `--test-args feature-gate/issue-43106-gating-of-inline.rs`
[00:50:19] error: 1 errors occurred comparing output.
[00:50:19] status: exit code: 101
[00:50:19] status: exit code: 101
[00:50:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-inline/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-inline/auxiliary" "-A" "unused"
[00:50:19] ------------------------------------------
[00:50:19] 
[00:50:19] ------------------------------------------
[00:50:19] stderr:
[00:50:19] stderr:
[00:50:19] ------------------------------------------
[00:50:19] {"message":"attribute should be applied to function or closure","code":{"code":"E0518","explanation":"\nThis error indicates that an `#[inline(..)]` attribute was incorrectly placed\non something other than a function or method.\n\nExamples of erroneous code:\n\n