plain
diff of stderr:

+ warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`
+ 
1 error: crate `NonSnakeCase` should have a snake case name
3    |

10 LL | #![deny(non_snake_case)]
11    |         ^^^^^^^^^^^^^^
---
To only update this specific test, also pass `--test-args lint/lint-non-snake-case-crate-dylib.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/lint-non-snake-case-crate-dylib.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-dylib" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-dylib/auxiliary"
stdout: none
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`


error: crate `NonSnakeCase` should have a snake case name
  --> fake-test-src-base/lint/lint-non-snake-case-crate-dylib.rs:2:18
   |
LL | #![crate_name = "NonSnakeCase"]
   |                  ^^^^^^^^^^^^ help: convert the identifier to snake case: `non_snake_case`
note: the lint level is defined here
  --> fake-test-src-base/lint/lint-non-snake-case-crate-dylib.rs:4:9
   |
LL | #![deny(non_snake_case)]
---
diff of stderr:

+ warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
+ 
1 error: crate `NonSnakeCase` should have a snake case name
3    |

10 LL | #![deny(non_snake_case)]
11    |         ^^^^^^^^^^^^^^
---
To only update this specific test, also pass `--test-args lint/lint-non-snake-case-crate-proc-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/lint-non-snake-case-crate-proc-macro.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-proc-macro" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-proc-macro/auxiliary"
stdout: none
warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`


error: crate `NonSnakeCase` should have a snake case name
  --> fake-test-src-base/lint/lint-non-snake-case-crate-proc-macro.rs:2:18
   |
LL | #![crate_name = "NonSnakeCase"]
   |                  ^^^^^^^^^^^^ help: convert the identifier to snake case: `non_snake_case`
note: the lint level is defined here
  --> fake-test-src-base/lint/lint-non-snake-case-crate-proc-macro.rs:4:9
   |
LL | #![deny(non_snake_case)]
