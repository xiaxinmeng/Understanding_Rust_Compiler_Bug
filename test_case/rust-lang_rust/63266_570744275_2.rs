sh
(lldb) b debug.rs:4
Breakpoint 1: 2 locations.
(lldb) r
Process 70294 launched: '/var/folders/3z/mjcq38mx3610kl4gnt9zw3gw0000gn/T/tmp.GzBm67H2/debugtest/main' (x86_64)
Process 70294 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100002ad3 main`main::main::h9e0ea2b04575dfdb at main.rs:4:4
   1   	fn main() {
   2   	    let v = vec![1, 2, 3];
   3   	    let p = std::path::PathBuf::from("/usr/bin");
-> 4   	    println!("v: {:?}", v);
   5   	    println!("p: {:?}", p);
   6   	}
Target 0: (main) stopped.
(lldb) p v
(alloc::vec::Vec<int>) $0 = vec![1, 2, 3]
(lldb) p p
Traceback (most recent call last):
  File "/Users/nbigaouette/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_rust_formatters.py", line 89, in print_val
    is_tuple_like = False)
  File "/Users/nbigaouette/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_rust_formatters.py", line 210, in print_struct_val
    body = separator.join([render_child(idx) for idx in range(field_start_index, len(fields))])
  File "/Users/nbigaouette/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_rust_formatters.py", line 203, in render_child
    return this + print_val(field_val.get_wrapped_value(), internal_dict)
TypeError: cannot concatenate 'str' and 'NoneType' objects
(std::path::PathBuf) $1 = {
  inner = None
}
