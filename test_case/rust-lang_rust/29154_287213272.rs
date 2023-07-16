
~ ❯ rust-lldb foo
(lldb) command source -s 0 '/tmp/rust-lldb-commands.jrvqZH'
Executing commands in '/tmp/rust-lldb-commands.jrvqZH'.
(lldb) command script import "/Users/sfackler/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_rust_formatters.py"
(lldb) type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
(lldb) type category enable Rust
(lldb) target create "foo"
Current executable set to 'foo' (x86_64).
(lldb) b thing_with_char
Breakpoint 1: where = foo`foo::thing_with_char + 11 at foo.rs:6, address = 0x000000010000178b
(lldb) r
Process 60614 launched: '/Users/sfackler/foo' (x86_64)
error: need to add support for DW_TAG_base_type 'char' encoded with DW_ATE = 0x8, bit_size = 32
Process 60614 stopped
* thread #1: tid = 0x40bc67, 0x000000010000178b foo`foo::thing_with_char(c=<unavailable>) + 11 at foo.rs:6, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x000000010000178b foo`foo::thing_with_char(c=<unavailable>) + 11 at foo.rs:6
   3   	    println!("{:?}", c);
   4   	}
   5
-> 6   	fn thing_with_char(c: char) -> Option<char> {
   7   	    thing_with_option_char(c)
   8   	}
   9
(lldb) n
Process 60614 stopped
* thread #1: tid = 0x40bc67, 0x0000000100001791 foo`foo::thing_with_char(c=<unavailable>) + 17 at foo.rs:7, queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100001791 foo`foo::thing_with_char(c=<unavailable>) + 17 at foo.rs:7
   4   	}
   5
   6   	fn thing_with_char(c: char) -> Option<char> {
-> 7   	    thing_with_option_char(c)
   8   	}
   9
   10  	fn thing_with_option_char(c: char) -> Option<char> {
(lldb) n
/Users/sfackler/.rustup/toolchains/stable-x86_64-apple-darwin/bin/rust-lldb: line 41: 60612 Floating point exception: 8   lldb --source-before-file="$TMPFILE" "$@"
~ ❯ lldb --version
lldb-360.1.70
