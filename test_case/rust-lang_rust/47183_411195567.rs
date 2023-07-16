
> attach <PID>
> thread list
> thread select 2
> bt
..Segmentation Fault...

error: xxx DWARF DIE at 0x000283c3 (class closure) has a member variable 0x000283ca (__0) whose type is a forward declaration, not a complete definition.
Try compiling the source file with -fno-limit-debug-info
$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rust-lldb: line 41: 98247 Segmentation fault: 11  lldb --source-before-file="$TMPFILE" "$@"

$ rustc --version
rustc 1.30.0-nightly (73c78734b 2018-08-05)
$ rust-lldb --version
lldb-902.0.79.7
  Swift-4.1
