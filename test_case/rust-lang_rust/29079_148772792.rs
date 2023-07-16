
$ cat foo.rs
fn main() {
    let a = 9.85;
    println!("{}", a);
}
$ rustc -g foo.rs
$ lldb ./foo
(lldb) target create "./foo"
bCurrent executable set to './foo' (x86_64).
(lldb) b foo.rs:3
Breakpoint 1: where = foo`foo::main + 107 at foo.rs:3, address = 0x000000010000156b
(lldb) r
Process 98510 launched: './foo' (x86_64)
Process 98510 stopped
* thread #1: tid = 0xdf51c, 0x000000010000156b foo`foo::main + 107 at foo.rs:3, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x000000010000156b foo`foo::main + 107 at foo.rs:3
   1    fn main() {
   2        let a = 9.85;
-> 3        println!("{}", a);
   4    }
(lldb) print a
(double) $0 = 9.8499999999999996
(lldb) 
