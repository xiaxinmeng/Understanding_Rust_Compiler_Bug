
$ rustc --target i686-unknown-linux-gnu --print cfg | grep sse2
target_feature="sse2"
$ clang -x c -target i686-unknown-linux-gnu -emit-llvm -S -o - - <<< "void foo() {}" | grep sse2
attributes #0 = { ... "target-features"="+fxsr,+mmx,+sse,+sse2,+x87" ... }
