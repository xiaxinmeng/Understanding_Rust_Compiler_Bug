bash
rustc +nightly --crate-name test_chalk --edition=2018 src/lib.rs --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=1ff6265fd5bdefac -C extra-filename=-1ff6265fd5bdefac --out-dir /tmp/test_chalk/target/debug/deps -C incremental=/tmp/test_chalk/target/debug/incremental -L dependency=/tmp/test_chalk/target/debug/deps -C target-cpu=native -Z chalk=yes
