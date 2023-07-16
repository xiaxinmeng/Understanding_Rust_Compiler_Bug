
   Compiling scgi_uploader v0.1.0 (file:///home/vi/code/scgi_uploader)
     Running `rustc src/main.rs --crate-name scgi_uploader --crate-type bin -g -C lto --out-dir /home/vi/code/scgi_uploader/target/debug --emit=dep-info,link -L dependency=/home/vi/code/scgi_uploader/target/debug -L dependency=/home/vi/code/scgi_uploader/target/debug/deps --extern time=/home/vi/code/scgi_uploader/target/debug/deps/libtime-9205cc1af6219594.rlib --extern scgi=/home/vi/code/scgi_uploader/target/debug/deps/libscgi-f7a16f1d604a0cb1.rlib`
rustc: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/CodeGen/AsmPrinter/DwarfUnit.cpp:724: llvm::DIE* llvm::DwarfUnit::getOrCreateTypeDIE(const llvm::MDNode*): Assertion `Ty == resolve(Ty->getRef()) && "type was not uniqued, possible ODR violation."' failed.
error: Could not compile `scgi_uploader`.

Caused by:
  Process didn't exit successfully: `rustc src/main.rs --crate-name scgi_uploader --crate-type bin -g -C lto --out-dir /home/vi/code/scgi_uploader/target/debug --emit=dep-info,link -L dependency=/home/vi/code/scgi_uploader/target/debug -L dependency=/home/vi/code/scgi_uploader/target/debug/deps --extern time=/home/vi/code/scgi_uploader/target/debug/deps/libtime-9205cc1af6219594.rlib --extern scgi=/home/vi/code/scgi_uploader/target/debug/deps/libscgi-f7a16f1d604a0cb1.rlib` (signal: 6, SIGABRT: process abort signal)
