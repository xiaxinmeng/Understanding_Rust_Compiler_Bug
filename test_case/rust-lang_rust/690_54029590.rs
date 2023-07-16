
make -C $RUST_SRC/src/compiler-rt" ProjSrcRoot="$RUST_SRC/src/compiler-rt" ProjObjRoot="$RUST_SRC/x86_64-apple-darwin/rt/compiler-rt" CC="clang" AR="ar" RANLIB="ar s" CFLAGS="-Wall -Werror -g -fPIC  -Qunused-arguments" TargetTriple=x86_64-apple-darwin clang_darwin-profile_osx
