
cd ${srcdir}
rm -rf src/llvm/tools/clang
rm -rf src/llvm/projects/compiler-rt # also extraneous, also likely to be present
rm -rf .git/modules/src/llvm/modules
cd ${builddir}
rm -rf llvm/*/tools/clang
rm -rf llvm/*/projects/compiler-rt
