plain
[RUSTC-TIMING] regex_automata test:false 10.399
[RUSTC-TIMING] regex test:false 10.838
[RUSTC-TIMING] salsa_macros test:false 2.927
   Compiling salsa v0.17.0-pre.2
rustc: /checkout/src/llvm-project/llvm/include/llvm/CodeGen/MachineInstr.h:506: llvm::MachineOperand& llvm::MachineInstr::getOperand(unsigned int): Assertion `i < getNumOperands() && "getOperand() out of range!"' failed.
[RUSTC-TIMING] tracing_attributes test:false 3.394
rustc exited with signal: 6 (SIGABRT) (core dumped)
error: could not compile `tracing-attributes`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name tracing_attributes --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-attributes-0.1.22/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg 'values(feature, "async-await")' --check-cfg 'names()' --check-cfg 'values()' -C metadata=94c3cd2c5427dc2a -C extra-filename=-94c3cd2c5427dc2a --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/release/deps -L dependency=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/release/deps --extern proc_macro2=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/release/deps/libproc_macro2-0fb922aca1091b45.rlib --extern quote=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/release/deps/libquote-2b2b5ad8408c0769.rlib --extern syn=/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools/release/deps/libsyn-c0b14e37bec48564.rlib --extern proc_macro --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] thiserror_impl test:false 3.258
[RUSTC-TIMING] pulldown_cmark test:false 11.288
[RUSTC-TIMING] notify test:false 8.944
[RUSTC-TIMING] regex_syntax test:false 16.336
