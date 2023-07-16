
elif isWindows && [[ ${CUSTOM_MINGW-0} -ne 1 ]]; then
    (...)
    mkdir -p citools
    cd citools
    curl -f "${MIRRORS_BASE}/LLVM-9.0.0-win64.tar.gz" | tar xzf -
    ciCommandSetEnv RUST_CONFIGURE_ARGS \
        "${RUST_CONFIGURE_ARGS} --set llvm.clang-cl=$(pwd)/clang-rust/bin/clang-cl.exe"
fi
