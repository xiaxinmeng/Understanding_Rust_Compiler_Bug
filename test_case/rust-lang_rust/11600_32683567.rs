
find . -name '*.rs' | xargs egrep '\.rc\>'
./src/librustc/middle/trans/base.rs:    // Append ".rc" to crate name as LLVM module identifier.
./src/librustc/middle/trans/base.rs:    let llmod_id = link_meta.crateid.name.clone() + ".rc";
./src/compiletest/compiletest.rs:    // Pretty-printer does not work with .rc files yet
./src/compiletest/compiletest.rs:          _ => ~[~".rc", ~".rs"]
./src/test/run-pass/item-attributes.rs:// for completeness since .rs files linked from .rc files support this
./src/test/run-pass/issue_3136_b.rs:// aux-build:issue_3136_a.rc
