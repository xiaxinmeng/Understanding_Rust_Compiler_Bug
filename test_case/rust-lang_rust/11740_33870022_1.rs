
11740.rs:7:9: 7:27 error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
11740.rs:7         self.attrs.iter().find(|attr| {
                   ^~~~~~~~~~~~~~~~~~
11740.rs:7:39: 10:10 note: first, the lifetime cannot outlive an anonymous lifetime defined on the block at 7:38...
11740.rs:7         self.attrs.iter().find(|attr| {
11740.rs:8                 let attr: () = std::cast::transmute(attr);
11740.rs:9                 true
11740.rs:10         });
11740.rs:8:53: 8:57 note: ...so that the pointer does not outlive the data it points at
11740.rs:8                 let attr: () = std::cast::transmute(attr);
                                                               ^~~~
11740.rs:7:9: 7:19 note: but, the lifetime must be valid for the expression at 7:8...
11740.rs:7         self.attrs.iter().find(|attr| {
                   ^~~~~~~~~~
11740.rs:7:9: 7:19 note: ...so that automatically reference is valid at the time of borrow
11740.rs:7         self.attrs.iter().find(|attr| {
