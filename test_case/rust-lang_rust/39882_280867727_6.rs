
ACCESS of size 0 at 0x7ffebd56e924 thread T0
    #0 0x55b1e38d425a in hello::main::h845ed3cde81856e0 /home/japaric/tmp/hello/src/main.rs:6
    #1 0x55b1e39cd30b in __rust_maybe_catch_panic /checkout/src/libpanic_abort/lib.rs:40

Address 0x7ffebd56e924 is located in stack of thread T0 at offset 36 in frame
    #0 0x55b1e38d404f in hello::main::h845ed3cde81856e0 /home/japaric/tmp/hello/src/main.rs:3

  This frame has 3 object(s):
    [32, 36) 'arg' <== Memory access at offset 36 is inside this variable
    [48, 52) '_7'
    [64, 68) 'x'
