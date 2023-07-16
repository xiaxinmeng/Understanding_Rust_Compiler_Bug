
---- [run-pass] run-pass/sepcomp-lib-lto.rs stdout ----

error: compilation failed!
status: signal: 6
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/simon/projects/rust/src/test/run-pass/sepcomp-lib-lto.rs -L x86_64-unknown-linux-gnu/test/run-pass/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/sepcomp-lib-lto.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -o x86_64-unknown-linux-gnu/test/run-pass/sepcomp-lib-lto.stage2-x86_64-unknown-linux-gnu --cfg rtopt -O -L x86_64-unknown-linux-gnu/rt -C lto
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
rustc: ../../../../../../projects/rust/src/llvm/lib/CodeGen/LexicalScopes.cpp:179: llvm::LexicalScope* llvm::LexicalScopes::getOrCreateRegularScope(llvm::MDNode*): Assertion `DISubprogram(Scope).describes(MF->getFunction())' failed.

------------------------------------------
