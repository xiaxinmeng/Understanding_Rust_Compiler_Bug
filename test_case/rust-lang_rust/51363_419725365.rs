plain
[00:46:30] ....................................................................................................
[00:46:32] ....................................................................................................
[00:46:35] ....................................................................................................
[00:46:37] ....................................................................................................
[00:46:39] .....................................................F..............................................
[00:46:43] .................F.................i................................................................
[00:46:46] ..i.................................................................................................
[00:46:52] .......................................................................................i............
[00:46:54] ....................................................................................................
[00:46:57] ....................................................................................................
[00:47:00] ....................................................................................................
---
[00:52:31] 
[00:52:31] ---- [ui] ui/feature-gates/feature-gate-linker-flavor.rs stdout ----
[00:52:31] diff of stderr:
[00:52:31] 
[00:52:31] - error[E0658]: the `#[used]` attribute is an experimental feature (see issue #40289)
[00:52:31] + error: attribute must be applied to a `static` variable
[00:52:31] 2   --> $DIR/feature-gate-linker-flavor.rs:16:1
[00:52:31] 3    |
[00:52:31] 4 LL | #[used]
[00:52:31] 5    | ^^^^^^^
[00:52:31] -    |
[00:52:31] -    = help: add #![feature(used)] to the crate attributes to enable
[00:52:31] 8 
---
[00:52:31] 12 
[00:52:31] 
[00:52:31] 
[00:52:31] The actual stderr differed from the expected stderr.
[00:52:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-linker-flavor/feature-gate-linker-flavor.stderr
[00:52:31] To update references, rerun the tests and pass the `--bless` flag
[00:52:31] To only update this specific test, also pass `--test-args feature-gates/feature-gate-linker-flavor.rs`
[00:52:31] error: 1 errors occurred comparing output.
[00:52:31] status: exit code: 1
[00:52:31] status: exit code: 1
[00:52:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-linker-flavor.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-linker-flavor/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-linker-flavor/auxiliary" "-A" "unused"
[00:52:31] ------------------------------------------
[00:52:31] 
[00:52:31] ------------------------------------------
[00:52:31] stderr:
[00:52:31] stderr:
[00:52:31] ------------------------------------------
[00:52:31] {"message":"attribute must be applied to a `static` variable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-linker-flavor.rs","byte_start":702,"byte_end":709,"line_start":16,"line_end":16,"column_start":1,"column_end":8,"is_primary":true,"text":[{"text":"#[used]","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"render
