
/tmp/rust % cargo new example
     Created binary (application) `example` package
/tmp/rust % cd example
rust/example % ls -la
drwxr-xr-x@   - k  6 Dec 19:25 .git
.rw-r--r--@   8 k  6 Dec 19:25 .gitignore
.rw-r--r--@ 176 k  6 Dec 19:25 Cargo.toml
drwxr-xr-x@   - k  6 Dec 19:25 src
rust/example % cargo build
   Compiling example v0.1.0 (/tmp/rust/example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
rust/example % lldb target/debug/example
Voltron loaded.
(lldb) target create "target/debug/example"
Current executable set to '/tmp/rust/example/target/debug/example' (x86_64).
(lldb) b main
Breakpoint 1: 2 locations.
(lldb) run
Process 88077 launched: '/tmp/rust/example/target/debug/example' (x86_64)
Process 88077 stopped
* thread #1, name = 'example', stop reason = breakpoint 1.2
    frame #0: 0x000055555555bc70 example`main
example`main:
->  0x55555555bc70 <+0>:  pushq  %rax
    0x55555555bc71 <+1>:  movq   %rsi, %rdx
    0x55555555bc74 <+4>:  leaq   0x3834c(%rip), %rax       ; __rustc_debug_gdb_scripts_section__
    0x55555555bc7b <+11>: movb   (%rax), %al
(lldb) c
Process 88077 resuming
Process 88077 stopped
* thread #1, name = 'example', stop reason = breakpoint 1.1
    frame #0: 0x000055555555bc34 example`example::main::h1644cce4bdca4434 at main.rs:2:5
   1   	fn main() {
-> 2   	    println!("Hello, world!");
   3   	}
   