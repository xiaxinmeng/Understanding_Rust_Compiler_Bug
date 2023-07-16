plain
.................................................................................................... 5600/12851
...................................................................i................................ 5700/12851
.................................................................................................... 5800/12851
.................................................................................................... 5900/12851
....................................................F............................................... 6000/12851
.............................F....................................................................F. 6100/12851
.............i...................................................................................... 6300/12851
.............................................................................................i...... 6400/12851
.................................................................................................... 6500/12851
..........................................................i......................................... 6600/12851
---
..............................................iii................................................... 12800/12851
...................................................
failures:

---- [ui] ui/associated-types/bound-lifetime-in-binding-only.rs#ok stdout ----

error in revision `ok`: /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:67: unexpected warning: '67:19: 67:63: where-clause bound is impossible to satisfy'

error in revision `ok`: /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:67: unexpected warning: '67:19: 67:63: where-clause bound is impossible to satisfy'

error in revision `ok`: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "ok" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.ok" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.ok/auxiliary"
    Error {
        line_num: 67,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "67:19: 67:63: where-clause bound is impossible to satisfy",
    Error {
        line_num: 67,
        kind: Some(
            Warning,
            Warning,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
        ),
        msg: "67:19: 67:63: where-clause bound is impossible to satisfy",
]


thread '[ui] ui/associated-types/bound-lifetime-in-binding-only.rs#ok' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13

---- [ui] ui/associated-types/issue-69398.rs stdout ----
normalized stderr:
warning: where-clause bound is impossible to satisfy
warning: where-clause bound is impossible to satisfy
  --> $DIR/issue-69398.rs:14:28
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args associated-types/issue-69398.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-69398.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-69398" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-69398/auxiliary"
stdout: none
--- stderr -------------------------------
warning: where-clause bound is impossible to satisfy
   |
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 1 warning emitted
------------------------------------------



---- [ui] ui/consts/issue-67696-const-prop-ice.rs stdout ----
normalized stderr:
warning: where-clause bound is impossible to satisfy
  --> $DIR/issue-67696-const-prop-ice.rs:13:33
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: where-clause bound is impossible to satisfy
  --> $DIR/issue-67696-const-prop-ice.rs:13:33
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: where-clause bound is impossible to satisfy
  --> $DIR/issue-67696-const-prop-ice.rs:13:33
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 3 warnings emitted



---
To only update this specific test, also pass `--test-args consts/issue-67696-const-prop-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-67696-const-prop-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-67696-const-prop-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--emit=mir,link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-67696-const-prop-ice/auxiliary"
stdout: none
--- stderr -------------------------------
warning: where-clause bound is impossible to satisfy
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/consts/issue-67696-const-prop-ice.rs:13:33
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/consts/issue-67696-const-prop-ice.rs:13:33
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 3 warnings emitted
------------------------------------------



---- [ui] ui/feature-gates/feature-gate-trivial_bounds.rs stdout ----

error: /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:65: unexpected warning: '65:32: 65:44: where-clause bound is impossible to satisfy'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds/auxiliary"
    Error {
        line_num: 65,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "65:32: 65:44: where-clause bound is impossible to satisfy",
]

thread '[ui] ui/feature-gates/feature-gate-trivial_bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13


---- [ui] ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs stdout ----

error: /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs:9: unexpected warning: '9:5: 9:51: where-clause bound is impossible to satisfy'

error: /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs:28: unexpected warning: '28:5: 28:51: where-clause bound is impossible to satisfy'
error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "9:5: 9:51: where-clause bound is impossible to satisfy",
    Error {
        line_num: 28,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "28:5: 28:51: where-clause bound is impossible to satisfy",
]

thread '[ui] ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13


---- [ui] ui/issues/issue-36839.rs stdout ----
normalized stderr:
warning: where-clause bound is impossible to satisfy
  --> $DIR/issue-36839.rs:14:28
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args issues/issue-36839.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36839.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36839" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36839/auxiliary"
stdout: none
--- stderr -------------------------------
warning: where-clause bound is impossible to satisfy
   |
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 1 warning emitted
------------------------------------------



---- [ui] ui/issues/issue-39970.rs stdout ----

error: /checkout/src/test/ui/issues/issue-39970.rs:15: unexpected warning: '15:5: 15:38: where-clause bound is impossible to satisfy'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39970.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970/auxiliary"
    Error {
        line_num: 15,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "15:5: 15:38: where-clause bound is impossible to satisfy",
]

thread '[ui] ui/issues/issue-39970.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13


---- [ui] ui/issues/issue-42796.rs stdout ----

error: /checkout/src/test/ui/issues/issue-42796.rs:9: unexpected warning: '9:40: 9:74: where-clause bound is impossible to satisfy'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42796.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "9:40: 9:74: where-clause bound is impossible to satisfy",
]

thread '[ui] ui/issues/issue-42796.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13


---- [ui] ui/mir/issue-91745.rs stdout ----
normalized stderr:
warning: where-clause bound is impossible to satisfy
  --> $DIR/issue-91745.rs:14:28
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args mir/issue-91745.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-91745.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-91745" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-91745/auxiliary"
stdout: none
--- stderr -------------------------------
warning: where-clause bound is impossible to satisfy
   |
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
warning: 1 warning emitted
------------------------------------------



---- [ui] ui/regions/regions-normalize-in-where-clause-list.rs stdout ----

error: /checkout/src/test/ui/regions/regions-normalize-in-where-clause-list.rs:28: unexpected error: '28:5: 28:38: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements [E0495]'
error: /checkout/src/test/ui/regions/regions-normalize-in-where-clause-list.rs:24: expected error not found: cannot infer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-normalize-in-where-clause-list.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-normalize-in-where-clause-list" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-normalize-in-where-clause-list/auxiliary"
    Error {
        line_num: 28,
        kind: Some(
            Error,
            Error,
        ),
        msg: "28:5: 28:38: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements [E0495]",
]

not found errors (from test file): [
    Error {
