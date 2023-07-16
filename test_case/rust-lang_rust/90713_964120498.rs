
error[E0309]: the parameter type `D` may not live long enough
  --> src/lib.rs:26:9
   |
20 | impl<T, D> GetOutput for T
   |         - help: consider adding an explicit lifetime bound...: `D: 'a`
...
26 |         self.deref().output()
   |         ^^^^^^^^^^^^ ...so that the type `D` is not borrowed for too long

error[E0309]: the parameter type `D` may not live long enough
  --> src/lib.rs:26:14
   |
20 | impl<T, D> GetOutput for T
   |         - help: consider adding an explicit lifetime bound...: `D: 'a`
...
26 |         self.deref().output()
   |              ^^^^^ ...so that the reference type `&D` does not outlive the data it points at

error[E0309]: the parameter type `D` may not live long enough
  --> src/lib.rs:26:22
   |
20 | impl<T, D> GetOutput for T
   |         - help: consider adding an explicit lifetime bound...: `D: 'a`
...
26 |         self.deref().output()
   |                      ^^^^^^ ...so that the reference type `&D` does not outlive the data it points at
