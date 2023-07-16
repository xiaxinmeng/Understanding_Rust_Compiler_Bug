sh
  ~/devspace/.other/chunk-list  stable $ cargo +stage2 miri test
Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)... done
   Compiling chunk-list v0.1.0 (/home/nimda/devspace/.other/chunk-list)
    Finished test [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/main.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/chunk_list-b3b731111ba842c5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
