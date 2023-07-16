
$ rustc +stage1 --print cfg --target aarch64-unknown-linux-gnu | grep target_feature
'+fp16' is not a recognized feature for this target (ignoring feature)
'+fp16' is not a recognized feature for this target (ignoring feature)
target_feature="f32mm"
target_feature="fhm"
target_feature="fp16"
target_feature="jsconv"
target_feature="llvm14-builtins-abi"
target_feature="neon"
target_feature="pmuv3"
target_feature="sve2"
target_feature="sve2-bitperm"
target_feature="sve2-sm4"
