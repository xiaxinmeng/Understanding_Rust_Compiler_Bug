shell
$ cargo run --target x86_64-unknown-linux-gnu                                                                                                                                                              
    Finished dev [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/x86_64-unknown-linux-gnu/debug/hash-test`
10274620341943426671
16828784525031417047

$ cross run --target powerpc-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s                                                                                                                                                                                 
     Running `qemu-ppc /target/powerpc-unknown-linux-gnu/debug/hash-test`
12793483990194865043
15425382868227043520
