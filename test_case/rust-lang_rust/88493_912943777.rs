plain
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> compiler/rustc_errors/src/diagnostic_builder.rs:245:21
    |
63  |             self.0.diagnostic.$n($($name),*);
...
...
245 |     forward!(pub fn set_is_lint(&mut self, is_lint: bool) -> &mut Self);
    |
note: associated function defined here
   --> compiler/rustc_errors/src/diagnostic.rs:567:12
    |
