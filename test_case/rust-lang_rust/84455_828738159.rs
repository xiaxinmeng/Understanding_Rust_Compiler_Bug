shell
> $ cargo build --verbose
>        Fresh ice_test v0.1.0 (/home/habbasi/bisect/ice_test)
>    Compiling bisect v0.1.0 (/home/habbasi/bisect)
>      Running `rustc --crate-name bisect --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=dffb54c3af23b7e8 -C extra-filename=-dffb54c3af23b7e8 --out-dir /home/habbasi/bisect/target/debug/deps -C incremental=/home/habbasi/bisect/target/debug/incremental -L dependency=/home/habbasi/bisect/target/debug/deps --extern ice_test=/home/habbasi/bisect/target/debug/deps/libice_test-e9c2aad897dc9d25.rlib`
> error: language item required, but not found: `eh_personality`
> 
> error: aborting due to previous error
> 
> error: could not compile `bisect`
> 
> Caused by:
>   process didn't exit successfully: `rustc --crate-name bisect --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=dffb54c3af23b7e8 -C extra-filename=-dffb54c3af23b7e8 --out-dir /home/habbasi/bisect/target/debug/deps -C incremental=/home/habbasi/bisect/target/debug/incremental -L dependency=/home/habbasi/bisect/target/debug/deps --extern ice_test=/home/habbasi/bisect/target/debug/deps/libice_test-e9c2aad897dc9d25.rlib` (exit status: 1)
> 