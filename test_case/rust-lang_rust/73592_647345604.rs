
   Compiling async-std v1.6.2 (/Users/dignifiedquire/opensource/async-std)
error[E0596]: cannot borrow `self` as mutable, as it is not declared as mutable
  --> src/io/stderr.rs:96:27
   |
92 |         self: Pin<&mut Self>,
   |         ---- help: consider changing this to be mutable: `mut self`
...
96 |         let state = &mut *self.0.lock().unwrap();
   |                           ^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `self` as mutable, as it is not declared as mutable
  --> src/io/stderr.rs:96:27
   |
92 |         self: Pin<&mut Self>,
   |         ---- help: consider changing this to be mutable: `mut self`
...
96 |         let state = &mut *self.0.lock().unwrap();
   |                           ^^^^ cannot borrow as mutable

error: aborting due to previous error
