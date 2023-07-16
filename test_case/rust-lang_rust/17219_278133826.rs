
Alexs-MacBook-Air-3:rust alex$ cat xx.rs
fn main() {
    println!("hi");
}

Alexs-MacBook-Air-3:rust alex$ rustc -C prefer-dynamic xx.rs
Alexs-MacBook-Air-3:rust alex$ ./xx
dyld: Library not loaded: @rpath/libstd-a38e14755045bae4.dylib
  Referenced from: /Users/alex/Programming/rust/rust/./xx
  Reason: image not found
Trace/BPT trap: 5
Alexs-MacBook-Air-3:rust alex$ otool -L xx
xx:
	@rpath/libstd-a38e14755045bae4.dylib (compatibility version 0.0.0, current version 0.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1197.1.1)
Alexs-MacBook-Air-3:rust alex$ rustc -C prefer-dynamic -C rpath xx.rs
Alexs-MacBook-Air-3:rust alex$ ./xx
hi
Alexs-MacBook-Air-3:rust alex$ otool -L xx
xx:
	@rpath/libstd-a38e14755045bae4.dylib (compatibility version 0.0.0, current version 0.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1197.1.1)
