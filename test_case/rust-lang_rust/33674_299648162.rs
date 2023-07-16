
$ lldb test
(lldb) target create "test"
Current executable set to 'test' (x86_64).
(lldb) source list
   1   	fn main() {
   2   		println!("Hello, world");
   3   	}
(lldb)
(lldb) b 2
Breakpoint 1: where = test`test::main + 24 at test.rs:2, address = 0x0000000100000b18
(lldb) r
Process 84869 launched: '/Users/mark/testing/test' (x86_64)
Process 84869 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100000b18 test`test::main at test.rs:2
   1   	fn main() {
-> 2   		println!("Hello, world");
   3   	}
