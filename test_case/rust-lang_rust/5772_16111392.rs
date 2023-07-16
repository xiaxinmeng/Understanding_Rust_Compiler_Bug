
progval@Andromede:~/src/rust-0.6$ RUST_LOG=rustc::back::link VERBOSE=1 RUST_LOG=rustc=1,::rt::backtrace make
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: using gcc
cfg: no pandoc found, omitting doc/rust.pdf
cfg: no llnextgen found, omitting grammar-verification
cfg: no pandoc found, omitting library doc build
x86_64-unknown-linux-gnu/stage0/bin/rustc --cfg stage0  -O  --target=x86_64-unknown-linux-gnu  -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so /home/progval/src/rust-0.6/src/librustc/rustc.rc && touch x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so
rust: task failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/libcore/run.rs:343
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(_ZN9rust_task13begin_failureEPKcS1_m+0x4f)[0x2b277d8d452f]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(+0x299f1)[0x2b277d8e49f1]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(upcall_fail+0x1c6)[0x2b277d8d6cb6]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x1059ab)[0x2b277a5cb9ab]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x105952)[0x2b277a5cb952]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(_ZN3sys12begin_unwind16_701b8ec61f97ead3_06E+0x71)[0x2b277a513441]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(_ZN3run14program_output16_3596fc6920d4f353_06E+0x2872)[0x2b277a5c5aa2]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x14e214)[0x2b277a614214]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN4back4link11link_binary17_483345abdae712883_06E+0x3a9d)[0x2b277bca4bad]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x5db835)[0x2b277ba6d835]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6driver6driver12compile_rest16_87c0f340aa21a393_06E+0x40e2)[0x2b277bd0b2b2]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8b9e84)[0x2b277bd4be84]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6driver6driver12compile_upto17_84743710faafb0af3_06E+0x108)[0x2b277bd0d738]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8b9e84)[0x2b277bd4be84]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN6driver6driver13compile_input15_a6b0fe22676e6b3_06E+0xca)[0x2b277bd0db7a]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN12run_compiler16_9de519bbeb758373_06E+0x20aa)[0x2b277bd3d31a]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8b7951)[0x2b277bd49951]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8b417c)[0x2b277bd4617c]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8b9e84)[0x2b277bd4be84]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0xce38e)[0x2b277a59438e]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x14e214)[0x2b277a614214]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(_Z18task_start_wrapperP10spawn_args+0x24)[0x2b277d8d5534]
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug
note: try running with RUST_LOG=rustc=1,::rt::backtrace to get further details and report the results to github.com/mozilla/rust/issues
rust: task failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/librustc/rustc.rc:357
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(_ZN9rust_task13begin_failureEPKcS1_m+0x4f)[0x2b277d8d452f]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(+0x299f1)[0x2b277d8e49f1]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(upcall_fail+0x1c6)[0x2b277d8d6cb6]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x1059ab)[0x2b277a5cb9ab]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x105952)[0x2b277a5cb952]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(_ZN3sys12begin_unwind16_701b8ec61f97ead3_06E+0x71)[0x2b277a513441]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so(+0x14e214)[0x2b277a614214]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN7monitor17_ab3b5d4d4b9efdb63_06E+0x1577)[0x2b277bd3f947]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(+0x8b9e84)[0x2b277bd4be84]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so(_ZN4main16_706f4ee7413ae583_06E+0x7e)[0x2b277bd4ba9e]
/home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so(_Z18task_start_wrapperP10spawn_args+0x24)[0x2b277d8d5534]
rust: domain main @0x13bb3c0 root task failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so] Erreur 101
