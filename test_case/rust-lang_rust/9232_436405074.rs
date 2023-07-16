
#0  rust_panic (msg=...) at libstd/panicking.rs:524
#1  0x00005555555891d9 in std::panicking::rust_panic_with_hook (payload=..., 
    message=..., file_line_col=0x5555555f1470) at libstd/panicking.rs:496
#2  0x0000555555562f78 in std::panicking::begin_panic (msg=..., 
    file_line_col=0x5555555f1470) at libstd/panicking.rs:410
#3  0x00005555555625dd in t::foo () at /tmp/t.rs:2
#4  0x00005555555625e6 in t::main () at /tmp/t.rs:6
