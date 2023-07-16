
hello.rs:11:5: 15:6 warning: method is never used: `method_bad2`, #[warn(dead_code)] on by default
hello.rs:11     fn method_bad2<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
hello.rs:12         let f: &Fn(()) -> bool = &f;
hello.rs:13 
hello.rs:14         self.method_bad2(|t| f.call((t,)))
hello.rs:15     }
hello.rs:7:5: 9:6 warning: function cannot return without recurring, #[warn(unconditional_recursion)] on by default
hello.rs:7     fn method_bad<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
hello.rs:8         self.method_bad(|t| f(t))
hello.rs:9     }
hello.rs:8:9: 8:34 note: recursive call site
hello.rs:8         self.method_bad(|t| f(t))
                   ^~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:7:5: 9:6 help: a `loop` may express intention better if this is on purpose
hello.rs:11:5: 15:6 warning: function cannot return without recurring, #[warn(unconditional_recursion)] on by default
hello.rs:11     fn method_bad2<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
hello.rs:12         let f: &Fn(()) -> bool = &f;
hello.rs:13 
hello.rs:14         self.method_bad2(|t| f.call((t,)))
hello.rs:15     }
hello.rs:14:9: 14:43 note: recursive call site
hello.rs:14         self.method_bad2(|t| f.call((t,)))
                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:11:5: 15:6 help: a `loop` may express intention better if this is on purpose
hello.rs:7:5: 9:6 error: reached the recursion limit during monomorphization
hello.rs:7     fn method_bad<F>(&self, f: F) -> bool where F: Fn(()) -> bool {
hello.rs:8         self.method_bad(|t| f(t))
hello.rs:9     }
