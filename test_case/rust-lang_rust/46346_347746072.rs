
durska:~ alex$ cargo new --bin xxx
     Created binary (application) `xxx` project
durska:~ alex$ cd xxx
durska:xxx alex$ cargo +beta build
   Compiling xxx v0.1.0 (file:///Users/alex/xxx)
    Finished dev [unoptimized + debuginfo] target(s) in 2.70 secs
durska:xxx alex$ ls target/debug
build		deps		examples	incremental	native		xxx		xxx.d		xxx.dSYM
durska:xxx alex$ rm -rf target
durska:xxx alex$ cargo +nightly build
   Compiling xxx v0.1.0 (file:///Users/alex/xxx)
    Finished dev [unoptimized + debuginfo] target(s) in 1.53 secs
durska:xxx alex$ ls target/debug
build		deps		examples	incremental	native		xxx		xxx.d
durska:xxx alex$
