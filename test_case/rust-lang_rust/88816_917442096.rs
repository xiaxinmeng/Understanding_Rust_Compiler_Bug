plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 641e02f388acc6b1d316a59c605a32d1711a8758 and 6f90f2990ff5e2f1ebb9e7fca0d864e54a42c4b1
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 Documenting implementors v0.1.0 (/checkout/src/test/rustdoc-gui/src/lib2/implementors)
 Documenting lib2 v0.1.0 (/checkout/src/test/rustdoc-gui/src/lib2)
    Finished dev [unoptimized + debuginfo] target(s) in 1.79s
   Compiling test_docs v0.1.0 (/checkout/src/test/rustdoc-gui/src/test_docs)
error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
   |
   |
10 |         output.push_str(&format!("/// Some const A{0}\npub const A{0}: isize = 0;\n", i);
   |                        ^ unclosed delimiter                                             ^ help: `)` may belong here
error: expected expression, found `)`
  --> build.rs:11:5
   |
11 |     }
