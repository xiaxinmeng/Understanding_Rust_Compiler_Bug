
compile_and_link: x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd.so
/home/brian/dev/rust2/src/libstd/ptr.rs:130:8: 130:51 warning: unused import [-W unused-imports (default)]
/home/brian/dev/rust2/src/libstd/ptr.rs:130     use memcpy64 = unstable::intrinsics::memmove64;
                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
rustc: /opt/dev/rust2/src/llvm/lib/IR/Attributes.cpp:119: llvm::StringRef llvm::Attribute::getKindAsString() const: Assertion `isStringAttribute() && "Invalid attribute type to get the kind as a string!"' failed.
Stack dump:
0.      Running pass 'Function Pass Manager' on module 'std.rc'.
1.      Running pass 'Module Verifier' on function '@_ZN8unstable10intrinsics12cosf32_1247416_54d9ca723652f656_07preE'
Aborted
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd.so] Error 134
