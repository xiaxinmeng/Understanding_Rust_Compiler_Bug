
$ x86_64-apple-darwin/stage2/bin/rustc /Users/dagit/local-data/rust-lang-compiler/src/test/run-pass/sepcomp-lib-lto.rs -L x86_64-apple-darwin/test/run-pass/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/run-pass/sepcomp-lib-lto.stage2-x86_64-apple-darwin.run-pass.libaux -o x86_64-apple-darwin/test/run-pass/sepcomp-lib-lto.stage2-x86_64-apple-darwin -O -L x86_64-apple-darwin/rt -C lto
Assertion failed: (Ty == resolve(Ty->getRef()) && "type was not uniqued, possible ODR violation."), function getOrCreateTypeDIE, file /Users/dagit/local-data/rust-lang-compiler/src/llvm/lib/CodeGen/AsmPrinter/DwarfUnit.cpp, line 713.
zsh: abort      x86_64-apple-darwin/stage2/bin/rustc  -L x86_64-apple-darwin/test/run-pass/
