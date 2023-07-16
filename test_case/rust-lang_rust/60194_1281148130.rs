console
$ patchelf --print-rpath build/x*64*linux*/llvm/lib/python*/site-packages/lldb/_lldb*.so | tr -t ':' '\n' | rg -v nix/store
$ORIGIN/../lib
