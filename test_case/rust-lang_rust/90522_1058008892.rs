
$ RUSTFLAGS="-C target-cpu=native -C profile-use=${PGO_DATA}/merged.profdata -Cllvm-args=-pgo-warn-missing-function" cargo build --release --features="tcmalloc_allocator"
(... regular compilation warnings ...)
module flag identifiers must be unique (or of 'require' type)
!"CG Profile"
LLVM ERROR: Broken module found, compilation aborted!
warning: `ogre-datasets-converter-demo` (bin "ogre-datasets-converter-demo") generated 18 warnings
error: could not compile `ogre-datasets-converter-demo`; 18 warnings emitted
