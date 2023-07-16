
titan:/tmp geofft$ rustup run nightly rustc --version
rustc 1.19.0-nightly (258ae6dd9 2017-06-15)
titan:/tmp geofft$ rustup run nightly rustc --crate-type=lib meal.rs
titan:/tmp geofft$ rustup run nightly rustc --crate-type staticlib diner.rs -L.
titan:/tmp geofft$ nm libdiner.a | grep libdiner
0000000000000000 T libdiner_actually_public_function
titan:/tmp geofft$ nm libdiner.a | grep burger
0000000000000000 T _ZN4meal6burger17h3224d595766d9dc8E
titan:/tmp geofft$ nm libdiner.a | grep fries
titan:/tmp geofft$ nm libdiner.a | grep drink
0000000000000000 T _ZN4meal5drink17h7624a2a00a5c235dE
titan:/tmp geofft$ nm libdiner.a | grep -c _ZN4core
589
titan:/tmp geofft$ rustup run nightly rustc --crate-type cdylib diner.rs -L.
titan:/tmp geofft$ nm -D libdiner.so 
                 w __cxa_finalize
                 w __gmon_start__
                 w _ITM_deregisterTMCloneTable
                 w _ITM_registerTMCloneTable
                 w _Jv_RegisterClasses
0000000000000570 T libdiner_actually_public_function
