
$ git bisect log
git bisect start
# good: [237bf3244fffef501cf37d4bda00e1fce3fcfb46] Auto merge of #59478 - Centril:rollup, r=Centril
git bisect good 237bf3244fffef501cf37d4bda00e1fce3fcfb46
# bad: [e782d790f1b63d82af39248bebe027f92d891bcc] Auto merge of #59522 - Centril:rollup, r=Centril
git bisect bad e782d790f1b63d82af39248bebe027f92d891bcc
# bad: [e782d790f1b63d82af39248bebe027f92d891bcc] Auto merge of #59522 - Centril:rollup, r=Centril
git bisect bad e782d790f1b63d82af39248bebe027f92d891bcc
# bad: [3df97f9da8e4a23772f5845bc641adae03e032be] Rollup merge of #59401 - japaric:compiler-builtins-stack-sizes, r=alexcrichton
git bisect bad 3df97f9da8e4a23772f5845bc641adae03e032be
# good: [b75b1655895e69f074936b73394a72e98aa067b9] Rollup merge of #59398 - phansch:rustfix_coverage, r=oli-obk
git bisect good b75b1655895e69f074936b73394a72e98aa067b9
# good: [8794e21ff329d1201d484c015d48e85490a64fa9] Rollup merge of #58019 - Zoxc:combine-late-lints, r=estebank
git bisect good 8794e21ff329d1201d484c015d48e85490a64fa9
# good: [6c8e3a5378e41e08a323139bb7aaa8a8823ec9bc] Remove unused variable
git bisect good 6c8e3a5378e41e08a323139bb7aaa8a8823ec9bc
# good: [f9262afa4d1a88715ff57907bd17eda4d039cea6] Rollup merge of #59394 - mark-i-m:dup-matcher-bindings-2, r=Centril
git bisect good f9262afa4d1a88715ff57907bd17eda4d039cea6
# bad: [7d365cf27f4249fc9b61ba8abfc813abe43f1cb7] compile all crates under test w/ -Zemit-stack-sizes
git bisect bad 7d365cf27f4249fc9b61ba8abfc813abe43f1cb7
# bad: [8b8488ce8fc047282e7159343f30609417f9fa39] bootstrap: build compiler-builtins with -Z emit-stack-sizes
git bisect bad 8b8488ce8fc047282e7159343f30609417f9fa39
# first bad commit: [8b8488ce8fc047282e7159343f30609417f9fa39] bootstrap: build compiler-builtins with 
