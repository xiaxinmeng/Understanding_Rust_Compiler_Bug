plain
.................................................................................................... 9500/11870
.................................................................................................... 9600/11870
.........................................................................i......i................... 9700/11870
.................................................................................................... 9800/11870
..................iiiiiii..iiiiii.i................................................................. 9900/11870
.................................................................................................... 10100/11870
.................................................................................................... 10200/11870
.................................................................................................... 10300/11870
.................................................................................................... 10400/11870
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.164 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.44s

 finished in 3.498 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- [ui] ui-fulldeps/lint-group-forbid-always-trumps-cli.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-forbid-always-trumps-cli.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "unused" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: allow(unused) incompatible with previous forbid
   |
   = note: `#[warn(forbidden_lint_groups)]` on by default
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line

warning: allow(unused) incompatible with previous forbid
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: for more information, see issue #81670 <https://github.com/rust-lang/rust/issues/81670>
   = note: `forbid` lint level was set on command line
warning: 80 warnings emitted


------------------------------------------
---
test result: FAILED. 66 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:15
