
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
