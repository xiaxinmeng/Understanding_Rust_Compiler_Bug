plain
---- src/lib.rs - (line 144) stdout ----
error: redundant field names in struct initialization
  --> src/lib.rs:157:25
   |
16 |     let graph = Graph { nodes: nodes, edges: edges };
   |                         ^^^^^^^^^^^^ help: replace it with: `nodes`
note: the lint level is defined here
  --> src/lib.rs:143:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_field_initializers)]` implied by `#[deny(warnings)]`

error: redundant field names in struct initialization
  --> src/lib.rs:157:39
   |
16 |     let graph = Graph { nodes: nodes, edges: edges };
   |                                       ^^^^^^^^^^^^ help: replace it with: `edges`
error: aborting due to 2 previous errors

Couldn't compile the test.
---- src/lib.rs - (line 206) stdout ----
---- src/lib.rs - (line 206) stdout ----
error: redundant field names in struct initialization
  --> src/lib.rs:219:25
   |
16 |     let graph = Graph { nodes: nodes, edges: edges };
   |                         ^^^^^^^^^^^^ help: replace it with: `nodes`
note: the lint level is defined here
  --> src/lib.rs:205:9
   |
2  | #![deny(warnings)]
2  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(redundant_field_initializers)]` implied by `#[deny(warnings)]`

error: redundant field names in struct initialization
  --> src/lib.rs:219:39
   |
error: test failed, to rerun pass '--doc'
16 |     let graph = Graph { nodes: nodes, edges: edges };
   |                                       ^^^^^^^^^^^^ help: replace it with: `edges`
error: aborting due to 2 previous errors

Couldn't compile the test.

---
test result: FAILED. 4 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_graphviz" "--" "--quiet"


Build completed unsuccessfully in 0:21:52
