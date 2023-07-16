bash
rustup toolchain install --profile minimal nightly
MINOR_VERSION=$(rustc +nightly --version | cut -d . -f 2)
LOWER_BOUND=44

llvm_version() {
    toolchain="$1"
    printf "Rust $toolchain    |    Clang "
    rustc +"$toolchain" -Vv | grep LLVM | cut -d ':' -f 2 | tr -d ' '
}

for version in `seq $LOWER_BOUND $((MINOR_VERSION - 2))`; do
    toolchain=1.$version.0
    rustup toolchain install --no-self-update --profile  minimal $toolchain >/dev/null 2>&1
    llvm_version $toolchain
done

for toolchain in beta nightly; do
    rustup toolchain install --no-self-update --profile minimal $toolchain >/dev/null 2>&1
    llvm_version $toolchain
done
