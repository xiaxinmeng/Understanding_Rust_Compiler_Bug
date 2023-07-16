
$ cargo +nightly test -- --report-time
    Finished test [unoptimized + debuginfo] target(s) in 0.16s
     Running target/debug/deps/coordinator-220d8b237f35464c
error: The "report-time" flag is only accepted on the nightly compiler
error: test failed, to rerun pass '--lib'

$ cargo test -- --report-time
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/coordinator-6d69e1ee36b403d9
error: Unrecognized option: 'report-time'
error: test failed, to rerun pass '--lib'
