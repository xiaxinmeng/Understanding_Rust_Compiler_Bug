plain
To only update this specific test, also pass `--test-args extenv/issue-55897.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/extenv/issue-55897.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/issue-55897" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extenv/issue-55897/auxiliary"
stdout: none
--- stderr -------------------------------
error: environment variable `NON_EXISTENT` not defined
   |
   |
LL |     include!(concat!(env!("NON_EXISTENT"), "/data.rs"));
   |
   = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)

error: suffixes on string literals are invalid
error: suffixes on string literals are invalid
  --> /checkout/tests/ui/extenv/issue-55897.rs:16:22
   |
LL |     include!(concat!("NON_EXISTENT"suffix, "/data.rs"));
   |                      ^^^^^^^^^^^^^^^^^^^^ invalid suffix `suffix`
error[E0432]: unresolved import `prelude`
  --> /checkout/tests/ui/extenv/issue-55897.rs:1:5
   |
   |
LL | use prelude::*; //~ ERROR unresolved import `prelude`
   |     |
   |     unresolved import
   |     help: a similar path exists: `std::prelude`


error[E0432]: unresolved import `env`
  --> /checkout/tests/ui/extenv/issue-55897.rs:4:9
   |
LL |     use env; //~ ERROR unresolved import `env`
   |         ^^^ no `env` in the root
help: consider importing this module instead
   |
LL |     use std::env
   |         ~~~~~~~~
   |         ~~~~~~~~

error: cannot determine resolution for the macro `env`
  --> /checkout/tests/ui/extenv/issue-55897.rs:6:22
   |
LL |     include!(concat!(env!("NON_EXISTENT"), "/data.rs"));
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0432`.
------------------------------------------
---

6    |
7 help: consider importing this type alias instead
8    |
- LL |     use A::B as _;
+ LL |     use A::B as _
11 
12 error[E0432]: unresolved import `crate::D::B2`

17    |
17    |
18 help: consider importing this type alias instead
19    |
- LL |     use A::B2;
+ LL |     use A::B2
22 
23 error: aborting due to 2 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/bad-import-with-rename/bad-import-with-rename.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/bad-import-with-rename.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/imports/bad-import-with-rename.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/bad-import-with-rename" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/bad-import-with-rename/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `crate::D::B`
   |
   |
LL |     use crate::D::B as _;
   |         ^^^^^^^^^^^^^^^^ no `B` in `D`
help: consider importing this type alias instead
   |
   |
LL |     use A::B as _

error[E0432]: unresolved import `crate::D::B2`
  --> /checkout/tests/ui/imports/bad-import-with-rename.rs:10:9
   |
---

6    |
7 help: consider importing this module instead
8    |
- LL | use glob_ok::something;
+ LL | use glob_ok::something
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-57015/issue-57015.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-57015.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/imports/issue-57015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-57015" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-57015/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `single_err::something`
   |
   |
LL | use single_err::something; //~ ERROR unresolved import `single_err::something`
   |     ^^^^^^^^^^^^^^^^^^^^^ no `something` in `single_err`
help: consider importing this module instead
   |
   |
LL | use glob_ok::something

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
---

6    |
7 help: consider importing one of these items instead
8    |
- LL |     use crate::m3::last_segment::issue_56125;
+ LL |     use crate::m3::last_segment::issue_56125
10    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
- LL |     use crate::m3::non_last_segment::non_last_segment::issue_56125;
+ LL |     use crate::m3::non_last_segment::non_last_segment::issue_56125
12    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
- LL |     use issue_56125::issue_56125;
+ LL |     use issue_56125::issue_56125
14    |         ~~~~~~~~~~~~~~~~~~~~~~~~
- LL |     use issue_56125::last_segment::issue_56125;
+ LL |     use issue_56125::last_segment::issue_56125
17      and 1 other candidate
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-56125/issue-56125.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-56125.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/imports/issue-56125.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-56125" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-56125/auxiliary" "--edition=2018" "--extern" "issue_56125"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `empty::issue_56125`
   |
   |
LL |     use empty::issue_56125; //~ ERROR unresolved import `empty::issue_56125`
   |         ^^^^^^^^^^^^^^^^^^ no `issue_56125` in `m3::empty`
help: consider importing one of these items instead
   |
LL |     use crate::m3::last_segment::issue_56125
   |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
---
LL |     use issue_56125::last_segment::issue_56125
   |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
     and 1 other candidate

error[E0659]: `issue_56125` is ambiguous
   |
   |
LL |     use issue_56125::last_segment::*;
   |         ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56125` could refer to a crate passed with `--extern`
   = help: use `::issue_56125` to refer to this crate unambiguously
note: `issue_56125` could also refer to the module imported here
   |
   |
LL |     use issue_56125::last_segment::*;
   = help: use `self::issue_56125` to refer to this module unambiguously


error[E0659]: `issue_56125` is ambiguous
   |
   |
LL |     use issue_56125::non_last_segment::non_last_segment::*;
   |         ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56125` could refer to a crate passed with `--extern`
   = help: use `::issue_56125` to refer to this crate unambiguously
note: `issue_56125` could also refer to the module imported here
   |
   |
LL |     use issue_56125::non_last_segment::non_last_segment::*;
   = help: use `self::issue_56125` to refer to this module unambiguously


error[E0659]: `issue_56125` is ambiguous
   |
   |
LL |     use issue_56125::*; //~ ERROR `issue_56125` is ambiguous
   |         ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56125` could refer to a crate passed with `--extern`
   = help: use `::issue_56125` to refer to this crate unambiguously
note: `issue_56125` could also refer to the module imported here
   |
   |
LL |     use issue_56125::*; //~ ERROR `issue_56125` is ambiguous
   = help: use `self::issue_56125` to refer to this module unambiguously

error: aborting due to 4 previous errors

---
To only update this specific test, also pass `--test-args rfc-2126-extern-absolute-paths/not-allowed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfc-2126-extern-absolute-paths/not-allowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/not-allowed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-extern-absolute-paths/not-allowed/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `alloc`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | use alloc; //~ ERROR unresolved import `alloc`
   |     ^^^^^ no external crate `alloc`
help: consider importing one of these items instead
   |
LL | use core::alloc
   |     ~~~~~~~~~~~
---
To only update this specific test, also pass `--test-args simd/portable-intrinsics-arent-exposed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/simd/portable-intrinsics-arent-exposed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/portable-intrinsics-arent-exposed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/portable-intrinsics-arent-exposed/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: maybe a missing crate `core`?
   |
   |
LL | use core::simd::intrinsics; //~ERROR E0433
   |     ^^^^ maybe a missing crate `core`?
   = help: consider adding `extern crate core` to use the `core` crate

error[E0432]: unresolved import `std::simd::intrinsics`
  --> /checkout/tests/ui/simd/portable-intrinsics-arent-exposed.rs:5:5
  --> /checkout/tests/ui/simd/portable-intrinsics-arent-exposed.rs:5:5
   |
LL | use std::simd::intrinsics; //~ERROR E0432
   |     ^^^^^^^^^^^^^^^^^^^^^ no `intrinsics` in `simd`
help: consider importing this module instead
   |
LL | use std::intrinsics
   |     ~~~~~~~~~~~~~~~
---
To only update this specific test, also pass `--test-args test-attrs/inaccessible-test-modules.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/test-attrs/inaccessible-test-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules/auxiliary" "--test"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `main`
   |
   |
LL | use main as x; //~ ERROR unresolved import `main`
   |     ^^^^^^^^^ no `main` in the root
error[E0432]: unresolved import `test`
  --> /checkout/tests/ui/test-attrs/inaccessible-test-modules.rs:6:5
   |
   |
LL | use test as y; //~ ERROR unresolved import `test`
   |     ^^^^^^^^^ no `test` in the root
help: consider importing this module instead
   |
LL | use test::test as y
   |     ~~~~~~~~~~~~~~~
---

6    |
7 help: consider importing this trait instead
8    |
- LL |     use a::Trait;
+ LL |     use a::Trait
11 
12 error[E0405]: cannot find trait `Trait` in this scope



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unresolved/unresolved-candidates/unresolved-candidates.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unresolved/unresolved-candidates.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unresolved/unresolved-candidates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unresolved/unresolved-candidates" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unresolved/unresolved-candidates/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `Trait`
   |
   |
LL |     use Trait; //~ ERROR unresolved import `Trait`
   |         ^^^^^ no `Trait` in the root
help: consider importing this trait instead
   |
LL |     use a::Trait
   |         ~~~~~~~~
   |         ~~~~~~~~

error[E0405]: cannot find trait `Trait` in this scope
  --> /checkout/tests/ui/unresolved/unresolved-candidates.rs:10:10
   |
LL |     impl Trait for () {} //~ ERROR cannot find trait `Trait` in this scope
   |
help: consider importing this trait
   |
   |
LL |     use a::Trait;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0405, E0432.
