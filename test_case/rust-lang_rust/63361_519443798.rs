
  = note: /usr/bin/ld: /home/mateusz/tmp/zen_test/target/debug/deps/zen_test-192eddefb9cd5b11.3p9u9xvxudj8dh99.rcgu.o: in function `core::sync::atomic::atomic_sub':
          /rustc/ad7c55e1fc55d9af4787b285cec1c64e3480ae84/src/libcore/sync/atomic.rs:2165: undefined reference to `__atomic_fetch_sub_8'
          /usr/bin/ld: /rustc/ad7c55e1fc55d9af4787b285cec1c64e3480ae84/src/libcore/sync/atomic.rs:2161: undefined reference to `__atomic_fetch_sub_8'
          /usr/bin/ld: /rustc/ad7c55e1fc55d9af4787b285cec1c64e3480ae84/src/libcore/sync/atomic.rs:2162: undefined reference to `__atomic_fetch_sub_8'
          /usr/bin/ld: /rustc/ad7c55e1fc55d9af4787b285cec1c64e3480ae84/src/libcore/sync/atomic.rs:2163: undefined reference to `__atomic_fetch_sub_8'
          /usr/bin/ld: /rustc/ad7c55e1fc55d9af4787b285cec1c64e3480ae84/src/libcore/sync/atomic.rs:2164: undefined reference to `__atomic_fetch_sub_8'
          collect2: error: ld returned 1 exit status
