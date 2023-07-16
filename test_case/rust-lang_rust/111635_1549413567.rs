
RUSTC_SYSROOT=$(rustc --print sysroot)
RUST_LLDB="$RUSTC_SYSROOT/lib/rustlib/$host/bin/lldb"

lldb=lldb
if [ -f "$RUST_LLDB" ]; then
    lldb="$RUST_LLDB"
...
