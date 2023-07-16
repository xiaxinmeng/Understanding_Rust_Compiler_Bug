
rustc --allow unused --crate-type=lib -Cno-prepopulate-passes --emit llvm-ir -o tmp.ll src/test/codegen/loads.rs && FileCheck-3.7 src/test/codegen/loads.rs < tmp.ll
