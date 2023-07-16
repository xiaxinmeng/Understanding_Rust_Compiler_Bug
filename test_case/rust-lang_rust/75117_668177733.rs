
$ env RUSTFLAGS=-Zsanitizer=memory cargo -Zbuild-std run --target x86_64-unknown-linux-gnu 
...
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/x86_64-unknown-linux-gnu/debug/issue_minimal`
Works: 
==27447==WARNING: MemorySanitizer: use-of-uninitialized-value
...
    #2 0x55bb619da926 in async_graph::splitter::_$LT$impl$u20$async_graph..Splitter$u20$for$u20$async_graph..SplitRec$LT$A$GT$$GT$::split::h4b9f7e0308aa3072 /../.cargo/git/checkouts/asyncgraph-9a60b9185121d673/e8ef35f/async_graph/src/splitter.rs:90:5
...
SUMMARY: MemorySanitizer: use-of-uninitialized-value
Exiting
