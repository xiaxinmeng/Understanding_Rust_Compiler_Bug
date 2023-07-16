plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4e880f8cbc191374ce7db335962489f41d6d4e3e and d343570a04975e0e322fc40d1420c13a0e051cff
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 Documenting implementors v0.1.0 (/checkout/src/test/rustdoc-gui/src/lib2/implementors)
 Documenting lib2 v0.1.0 (/checkout/src/test/rustdoc-gui/src/lib2)
    Finished dev [unoptimized + debuginfo] target(s) in 1.94s
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
