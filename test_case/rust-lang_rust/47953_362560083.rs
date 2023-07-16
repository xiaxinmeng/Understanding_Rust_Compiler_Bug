
paulg@debian ~/P/r/rust> ./x.py build
Updating submodules
Submodule path 'src/binaryen': checked out '1c9bf65aa0e371b84755a8ddd6e79497fac57171'
Submodule path 'src/binaryen/test/emscripten': checked out '1d979a75722a513b586e6705d662ff4ee0ea832b'
Submodule path 'src/binaryen/test/spec': checked out '4b50c1c3f0d28c1a848866c6cdd0a1f469e1923c'
Submodule path 'src/binaryen/test/waterfall': checked out '900646fc880d526160b0df9b78bc9dd4f02dc1ab'
Submodule path 'src/dlmalloc': checked out 'a2b424b600235af58f453577c2da1b0e1de2ffa5'
Submodule path 'src/doc/book': checked out '194eb8d5f1753fb5f4501011cebdc1b585712474'
Submodule path 'src/doc/nomicon': checked out 'fec3182d0b0a3cf8122e192b3270064a5b19be5b'
Submodule path 'src/doc/reference': checked out '1d791b55b23ec5389fbd5b3cee80db3f8bbdd162'
Submodule path 'src/doc/rust-by-example': checked out '4ebb8169dfe569b3dcbeab560607800bb717978a'
Submodule path 'src/jemalloc': checked out '1f5a28755e301ac581e2048011e4e0ff3da482ef'
Submodule path 'src/libcompiler_builtins': checked out '345447948f7a51eca970fa036cefd613d54a4f79'
fatal: Not a git repository: /home/paulg/Projets/rust-lang/rust/src/libcompiler_builtins/compiler-rt/../../../.git/modules/src/libcompiler_builtins/modules/compiler-rt
Failed to clone 'compiler-rt'. Retry scheduled
BUG: submodule considered for cloning, doesn't need cloning any more?
fatal: Not a git repository: /home/paulg/Projets/rust-lang/rust/src/libcompiler_builtins/compiler-rt/../../../.git/modules/src/libcompiler_builtins/modules/compiler-rt
Unable to fetch in submodule path 'src/libcompiler_builtins/compiler-rt'
Submodule path 'src/liblibc': checked out '56444a4545bd71430d64b86b8a71714cfdbe9f5d'
Submodule path 'src/llvm': checked out '860c10e60cff73073d20110ff5f130ef6c643210'
Submodule path 'src/rt/hoedown': checked out 'da282f1bb7277b4d30fa1599ee29ad8eb4dd2a92'
Submodule path 'src/tools/cargo': checked out '1d6dfea44f97199d5d5c177c7dadcde393eaff9a'
Submodule path 'src/tools/clippy': checked out '7d7fef1690218bbb406cf3bcadf7bb29dbb40cc5'
Submodule path 'src/tools/miri': checked out '919604e1ead8294c8ca14f101be4380ea1ea370c'
Submodule path 'src/tools/rls': checked out '511321ae1c2fa3f0e334885fecf406dd6c882836'
Submodule path 'src/tools/rust-installer': checked out 'b55e0fc77590cf5d23a01dedeb2104d8cbb48efc'
Submodule path 'src/tools/rust-installer/test/rust-installer-v1': checked out 'aed73472416064642911af790b25d57c9390b6c7'
Submodule path 'src/tools/rust-installer/test/rust-installer-v2': checked out 'e577c97b494be2815b215e3042207d6d4b7c5516'
Submodule path 'src/tools/rust-installer/test/rust-installer-v2/test/rust-installer-v1': checked out 'aed73472416064642911af790b25d57c9390b6c7'
Submodule path 'src/tools/rustfmt': checked out 'e0e3e22248cd14ebbe0253e9720261a0328bfc59'
Failed to recurse into submodule path 'src/libcompiler_builtins'
failed to run: git submodule update --init --recursive src/llvm src/rt/hoedown src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/doc/book src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/binaryen src/doc/rust-by-example
Build completed unsuccessfully in 0:00:04
