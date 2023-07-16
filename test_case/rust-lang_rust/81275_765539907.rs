plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.073 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i....iii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.398 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"src/doc"}) not skipped for "bootstrap::doc::Standalone" -- not in ["src/tools/tidy"]
Set({"library/alloc", "library/core", "library/panic_abort", "library/panic_unwind", "library/proc_macro", "library/std", "library/term", "library/test", "library/unwind"}) not skipped for "bootstrap::doc::Std" -- not in ["src/tools/tidy"]
Documenting stage2 std (x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
time: 0.558; rss: 1221MB create_renderer
time: 0.013; rss: 1221MB renderer_after_krate
   Compiling cc v1.0.60
    Checking core v0.0.0 (/checkout/library/core)
   Compiling compiler_builtins v0.1.39
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
 Documenting alloc v0.0.0 (/checkout/library/alloc)
time: 0.092; rss: 285MB create_renderer
time: 0.000; rss: 286MB renderer_after_krate
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking alloc v0.0.0 (/checkout/library/alloc)
---
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
 Documenting std v0.0.0 (/checkout/library/std)
time: 0.217; rss: 466MB create_renderer
time: 0.002; rss: 470MB renderer_after_krate
   Compiling std v0.0.0 (/checkout/library/std)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
time: 0.039; rss: 290MB create_renderer
time: 0.000; rss: 290MB renderer_after_krate
    Checking rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
    Checking term v0.0.0 (/checkout/library/term)
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
 Documenting test v0.0.0 (/checkout/library/test)
time: 0.036; rss: 274MB create_renderer
time: 0.000; rss: 274MB renderer_after_krate
Set({"compiler/rustc"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Documenting stage2 compiler (x86_64-unknown-linux-gnu)
Set({"compiler/rustc_apfloat"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_arena"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 97 tests
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..................F........F....F.F....F..F.FFFFFF....FF........FF..F..FFFF.......FF....F..F..FF.

---- [ui] rustdoc-ui/deprecated-attrs.rs stdout ----
normalized stdout:
normalized stdout:
time: 0.013; rss: 182MB\tcreate_renderer
time: 0.000; rss: 182MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/deprecated-attrs.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/deprecated-attrs.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deprecated-attrs.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deprecated-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/auxiliary"
------------------------------------------
------------------------------------------
time: 0.013; rss: 182MB create_renderer
time: 0.000; rss: 182MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: the `#![doc(no_default_passes)]` attribute is considered deprecated
   = warning: see issue #44136 <https://github.com/rust-lang/rust/issues/44136> for more information
   = warning: see issue #44136 <https://github.com/rust-lang/rust/issues/44136> for more information
   = help: you may want to use `#![doc(document_private_items)]`

warning: the `#![doc(passes = "...")]` attribute is considered deprecated
   = warning: see issue #44136 <https://github.com/rust-lang/rust/issues/44136> for more information

warning: 2 warnings emitted



------------------------------------------


---- [ui] rustdoc-ui/deref-recursive-cycle.rs stdout ----
normalized stdout:
time: 0.016; rss: 281MB\tcreate_renderer
time: 0.000; rss: 281MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deref-recursive-cycle/deref-recursive-cycle.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deref-recursive-cycle.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deref-recursive-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deref-recursive-cycle" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deref-recursive-cycle/auxiliary"
------------------------------------------
------------------------------------------
time: 0.016; rss: 281MB create_renderer
time: 0.000; rss: 281MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/async.rs stdout ----
normalized stdout:
time: 0.015; rss: 280MB\tcreate_renderer
time: 0.000; rss: 280MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/async/async.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/async/async.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/async.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/async" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/async/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 280MB create_renderer
time: 0.000; rss: 280MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/closure.rs stdout ----
normalized stdout:
time: 0.016; rss: 282MB\tcreate_renderer
time: 0.000; rss: 282MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/closure/closure.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/closure/closure.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/closure" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/closure/auxiliary"
------------------------------------------
------------------------------------------
time: 0.016; rss: 282MB create_renderer
time: 0.000; rss: 282MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/const-generics.rs stdout ----
normalized stdout:
time: 0.015; rss: 280MB\tcreate_renderer
time: 0.000; rss: 280MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/const-generics/const-generics.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/const-generics/const-generics.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/const-generics.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/const-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/const-generics" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/const-generics/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 280MB create_renderer
time: 0.000; rss: 280MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/generic-argument.rs stdout ----
normalized stdout:
time: 0.014; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/generic-argument/generic-argument.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/generic-argument/generic-argument.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/generic-argument.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/generic-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/generic-argument" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/generic-argument/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/impl-keyword-closure.rs stdout ----
normalized stdout:
time: 0.015; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword-closure/impl-keyword-closure.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword-closure/impl-keyword-closure.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/impl-keyword-closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/impl-keyword-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword-closure" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword-closure/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/impl-keyword.rs stdout ----
normalized stdout:
time: 0.014; rss: 280MB\tcreate_renderer
time: 0.000; rss: 280MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword/impl-keyword.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword/impl-keyword.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/impl-keyword.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/impl-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/impl-keyword/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 280MB create_renderer
time: 0.000; rss: 280MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/realistic-async.rs stdout ----
normalized stdout:
time: 0.014; rss: 282MB\tcreate_renderer
time: 0.000; rss: 282MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/realistic-async/realistic-async.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/realistic-async/realistic-async.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/realistic-async.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/realistic-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/realistic-async" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/realistic-async/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 282MB create_renderer
time: 0.000; rss: 282MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/trait-alias-closure.rs stdout ----
normalized stdout:
time: 0.014; rss: 280MB\tcreate_renderer
time: 0.000; rss: 280MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias-closure/trait-alias-closure.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/trait-alias-closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/trait-alias-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias-closure" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias-closure/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 280MB create_renderer
time: 0.000; rss: 280MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/error-in-impl-trait/trait-alias.rs stdout ----
normalized stdout:
time: 0.015; rss: 280MB\tcreate_renderer
time: 0.000; rss: 280MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias/trait-alias.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias/trait-alias.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-in-impl-trait/trait-alias.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/error-in-impl-trait/trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/error-in-impl-trait/trait-alias/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 280MB create_renderer
time: 0.000; rss: 280MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/ignore-block-help.rs stdout ----
normalized stdout:
time: 0.015; rss: 278MB\tcreate_renderer
time: 0.000; rss: 278MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/ignore-block-help/ignore-block-help.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args ignore-block-help.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/ignore-block-help.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/ignore-block-help" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/ignore-block-help/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 278MB create_renderer
time: 0.000; rss: 278MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------
warning: could not parse code block as Rust code
warning: could not parse code block as Rust code
  --> /checkout/src/test/rustdoc-ui/ignore-block-help.rs:3:5
   |
LL |   /// 