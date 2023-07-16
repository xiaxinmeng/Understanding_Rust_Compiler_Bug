plain
normalized stderr:
warning: found module declaration for lib.rs
  --> $DIR/issue-97007.rs:6:1
   |
LL | / mod lib {
LL | |     const N_ISLANDS: usize = 4;
LL | |     const N_BRIDGES: usize = 7;
LL | |     const BRIDGES: [(usize, usize); 7] = [(0, 1), (0, 1), (0, 2), (0, 3), (0, 3), (1, 2), (2, 3)];
LL | |     }
LL | | }
   | |_^
   |
   |
   = note: `#[warn(special_module_name)]` on by default
   = note: lib.rs is the root of this crate's library target
   = help: to refer to it from other targets, use the library's name as the path
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args const-generics/issue-97007.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-97007.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-97007" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-97007/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issue-97007.rs:6:1
   |
LL | / mod lib {
LL | / mod lib {
LL | |     const N_ISLANDS: usize = 4;
LL | |     const N_BRIDGES: usize = 7;
LL | |     const BRIDGES: [(usize, usize); 7] = [(0, 1), (0, 1), (0, 2), (0, 3), (0, 3), (1, 2), (2, 3)];
LL | |     }
LL | | }
   | |_^
   |
   |
   = note: `#[warn(special_module_name)]` on by default
   = note: lib.rs is the root of this crate's library target
   = help: to refer to it from other targets, use the library's name as the path
warning: 1 warning emitted
------------------------------------------


