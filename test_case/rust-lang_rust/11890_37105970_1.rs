 bash
$ rustc -Z lto foo.rs
rustc: /var/tmp/paludis/build/dev-lang-rust-scm/work/rust-scm/src/llvm/lib/Transforms/Utils/ValueMapper.cpp:194: void llvm::RemapInstruction(llvm::Instruction*, llvm::ValueToValueMapTy&, llvm::RemapFlags, llvm::ValueMapTypeRemapper*, llvm::ValueMaterializer*): Assertion `(Flags & RF_IgnoreMissingEntries) && "Referenced value not in value map!"' failed.
[1]    31382 abort (core dumped)  rustc -Z lto foo.rs
