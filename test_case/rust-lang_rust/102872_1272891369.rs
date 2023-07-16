plain
    Checking cranelift-native v0.87.0
    Checking cranelift-frontend v0.87.0
    Checking cranelift-object v0.87.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0026]: variant `rustc_target::abi::Variants::Multiple` does not have a field named `tag`
   |
23 |             tag: _,
   |             ^^^
   |             |
   |             |
   |             variant `rustc_target::abi::Variants::Multiple` does not have this field
   |             help: `rustc_target::abi::Variants::Multiple` has a field named `scalar`
error[E0027]: pattern does not mention field `scalar`
  --> src/discriminant.rs:22:9
   |
22 | /         Variants::Multiple {
---
   | |_________^ missing field `scalar`
   |
help: include the missing field in the pattern
   |
26 |             variants: _, scalar } => {
   |                        ~~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
26 |             variants: _, .. } => {


error[E0026]: variant `rustc_target::abi::Variants::Multiple` does not have a field named `tag`
   |
43 |             tag: _,
   |             ^^^
   |             |
   |             |
   |             variant `rustc_target::abi::Variants::Multiple` does not have this field
   |             help: `rustc_target::abi::Variants::Multiple` has a field named `scalar`
error[E0027]: pattern does not mention field `scalar`
  --> src/discriminant.rs:42:9
   |
42 | /         Variants::Multiple {
42 | /         Variants::Multiple {
43 | |             tag: _,
44 | |             field,
45 | |             encoding: TagEncoding::Niche { untagged_variant, ref tagged_variants, tags_start },
46 | |             variants: _,
47 | |         } => {
   | |_________^ missing field `scalar`
help: include the missing field in the pattern
   |
   |
46 |             variants: _, scalar } => {
   |                        ~~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
46 |             variants: _, .. } => {


error[E0026]: variant `rustc_target::abi::Variants::Multiple` does not have a field named `tag`
   |
   |
94 |         Variants::Multiple { tag, field, encoding, variants: _ } => {
   |                              |
   |                              |
   |                              variant `rustc_target::abi::Variants::Multiple` does not have this field
   |                              help: `rustc_target::abi::Variants::Multiple` has a field named `scalar`
error[E0027]: pattern does not mention field `scalar`
  --> src/discriminant.rs:94:9
   |
   |
94 |         Variants::Multiple { tag, field, encoding, variants: _ } => {
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `scalar`
help: include the missing field in the pattern
   |
   |
94 |         Variants::Multiple { tag, field, encoding, variants: _, scalar } => {
   |                                                               ~~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
94 |         Variants::Multiple { tag, field, encoding, variants: _, .. } => {

Some errors have detailed explanations: E0026, E0027.
For more information about an error, try `rustc --explain E0026`.
error: could not compile `rustc_codegen_cranelift` due to 6 previous errors
