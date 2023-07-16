
running 1 test
[DEBUG compiletest::runtest] running "/Users/eric/Proj/rust/rust/src/test/codegen/issue-44056-macos-tls-align.rs"
[DEBUG compiletest::header] compile-flags:  -O
[DEBUG compiletest::util] executing "/Users/eric/Proj/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/eric/Proj/rust/rust/src/test/codegen/issue-44056-macos-tls-align.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-C" "prefer-dynamic" "--out-dir" "/Users/eric/Proj/rust/rust/build/x86_64-apple-darwin/test/codegen/issue-44056-macos-tls-align" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/eric/Proj/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-O" "-L" "/Users/eric/Proj/rust/rust/build/x86_64-apple-darwin/test/codegen/issue-44056-macos-tls-align/auxiliary" "--emit=llvm-ir"
[DEBUG compiletest::util] executing "/Users/eric/Proj/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--input-file" "/Users/eric/Proj/rust/rust/build/x86_64-apple-darwin/test/codegen/issue-44056-macos-tls-align/issue-44056-macos-tls-align.ll" "/Users/eric/Proj/rust/rust/src/test/codegen/issue-44056-macos-tls-align.rs"
