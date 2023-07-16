 bash
case "$CFG_C_COMPILER" in
    ("ccache clang")
    ...
    ;;
    ("gcc")
    ...
    ;;
    # etc
esac

case "$CFG_CPUTYPE" in
    (x86*)
    LLVM_CXX_32="$LLVM_CXX_32 -m32"
    LLVM_CC_32="$LLVM_CC_32 -m32"
    ;;
esac
