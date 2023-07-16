plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
---- [rustdoc] tests/rustdoc/multiple-import-levels.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/multiple-import-levels/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/multiple-import-levels" "--deny" "warnings" "/checkout/tests/rustdoc/multiple-import-levels.rs"
stdout: none
--- stderr -------------------------------
error: doc comments on non-inlined reexports of local items are ignored
   |
12 |     /// 2
   |     ^^^^^
   |
   |
   = note: the documentation won't be visible because reexports don't have their own page
   = note: `-D rustdoc::unused-reexport-documentation` implied by `-D warnings`
help: try adding `#[doc(inline)]`
   |
13 |     #[doc(inline)] pub use crate::a::Type;
help: or move the documentation directly on the reexported item
   |
12 -     /// 2
12 +     
12 +     
   |

error: doc comments on non-inlined reexports of local items are ignored
   |
17 |     /// 3
   |     ^^^^^
   |
   |
   = note: the documentation won't be visible because reexports don't have their own page
help: try adding `#[doc(inline)]`
   |
18 |     #[doc(inline)] pub use crate::b::Type;
help: or move the documentation directly on the reexported item
   |
17 -     /// 3
17 +     
17 +     
   |

error: doc comments on non-inlined reexports of local items are ignored
   |
19 |     /// 4
   |     ^^^^^
   |
   |
   = note: the documentation won't be visible because reexports don't have their own page
help: try adding `#[doc(inline)]`
   |
20 |     #[doc(inline)] pub use crate::b::Type as Woof;
help: or move the documentation directly on the reexported item
   |
19 -     /// 4
19 +     
