plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# `-Ccodegen-units=16 -Copt-level=2` is used here to trigger thin LTO
# across codegen units to test deduplication of the named metadata
# (see `LLVMRustPrepareThinLTOImport` for details).
echo 'fn main(){}' | LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-ident/llvm-ident:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-ident/llvm-ident -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-ident/llvm-ident  - --emit=link,obj -Csave-temps -Ccodegen-units=16 -Copt-level=2
# `llvm-dis` is used here since `--emit=llvm-ir` does not emit LLVM IR
# for temporary outputs.
"/usr/lib/llvm-12/bin/llvm-dis" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-ident/llvm-ident/*.bc
--- stderr -------------------------------
--- stderr -------------------------------
warning: ignoring emit path because multiple .o files were produced
warning: 1 warning emitted


llvm-dis: Too many positional arguments specified!
Can specify at most 1 positional arguments: See: /usr/lib/llvm-12/bin/llvm-dis --help
make: *** [Makefile:12: all] Error 1



failures:
