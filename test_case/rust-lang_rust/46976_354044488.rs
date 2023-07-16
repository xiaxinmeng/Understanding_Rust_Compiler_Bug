
% rust-lldb ./stage1/bin/rustc file.rs
(lldb) ---8< ---
(lldb) breakpoint set --func-regex ".*<METHOD_NAME>.*"
(lldb) run
(lldb) bt
 --- 8< ---
