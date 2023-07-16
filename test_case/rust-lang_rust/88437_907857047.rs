plain
    Finished dev [unoptimized + debuginfo] target(s) in 33.86s
+ RUST_LOG=collector=debug
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt,Doc --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo,token-stream-stress,match-stress-enum
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt,Doc --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo,token-stream-stress,match-stress-enum`
collector error: 'Doc' build specified but '--rustdoc' not specified and no 'rustdoc' found next to 'rustc'
