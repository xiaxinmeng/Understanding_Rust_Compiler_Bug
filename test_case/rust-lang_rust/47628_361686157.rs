sh
~/serde ((v1.0.27)):160$ cargo clean && time cargo +nightly-2017-12-24 build -vv
   Compiling serde v1.0.27 (file:///~/serde/serde)
     Running `rustc --crate-name serde src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=7c335e5d4a9ee66d -C extra-filename=-7c335e5d4a9ee66d --out-dir /~/serde/target/debug/deps -L dependency=/~/serde/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 9.50 secs

real	0m9.752s
user	0m9.463s
sys	0m0.409s

~/serde ((v1.0.27)):161$ cargo clean && time cargo +nightly-2017-12-25 build -vv
   Compiling serde v1.0.27 (file:///~/serde/serde)
     Running `rustc --crate-name serde serde/src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=633312819f55a634 -C extra-filename=-633312819f55a634 --out-dir /~/serde/target/debug/deps -C incremental=/~/serde/target/debug/incremental -L dependency=/~/serde/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 12.42 secs

real	0m13.098s
user	0m11.421s
sys	0m0.814s

~/serde ((v1.0.27)):162$ cargo clean && time cargo +16992930835ce3376a4aaed42307726e1fc78e45 build -vv
   Compiling serde v1.0.27 (file:///~/serde/serde)
     Running `rustc --crate-name serde serde/src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=7a2f2aaec6829696 -C extra-filename=-7a2f2aaec6829696 --out-dir /~/serde/target/debug/deps -C incremental=/~/serde/target/debug/incremental -L dependency=/~/serde/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 12.82 secs

real	0m13.442s
user	0m11.809s
sys	0m0.781s
