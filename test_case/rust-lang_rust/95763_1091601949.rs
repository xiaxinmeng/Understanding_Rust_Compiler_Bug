plain
   Compiling rustc-demangle v0.1.21
error: lifetime may not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/v0.rs:954:21
     |
632  |   impl<'a, 'b, 's> Printer<'a, 'b, 's> {
     |        --      -- lifetime `'s` defined here
     |        |
     |        lifetime `'a` defined here
...
954  |               b'F' => self.in_binder(|this| {
     |  _____________________^
955  | |                 let is_unsafe = this.eat(b'U');
956  | |                 let abi = if this.eat(b'K') {
957  | |                     if this.eat(b'C') {
1000 | |                 Ok(())
1001 | |             })?,
1001 | |             })?,
     | |______________^ argument requires that `'a` must outlive `'s`
     |
     = help: consider adding the following bound: `'a: 's`
     = note: requirement occurs because of a mutable reference to `Printer<'_, '_, '_>`
     = note: mutable references are invariant over their type parameter
     = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/v0.rs:954:21
     |
     |
632  |   impl<'a, 'b, 's> Printer<'a, 'b, 's> {
     |        --      -- lifetime `'s` defined here
     |        |
     |        lifetime `'a` defined here
...
954  |               b'F' => self.in_binder(|this| {
     |  _____________________^
955  | |                 let is_unsafe = this.eat(b'U');
956  | |                 let abi = if this.eat(b'K') {
957  | |                     if this.eat(b'C') {
1000 | |                 Ok(())
1001 | |             })?,
1001 | |             })?,
     | |______________^ argument requires that `'s` must outlive `'a`
     |
     = help: consider adding the following bound: `'s: 'a`
     = note: requirement occurs because of a mutable reference to `Printer<'_, '_, '_>`
     = note: mutable references are invariant over their type parameter
     = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/v0.rs:990:17
    |
    |
632 | impl<'a, 'b, 's> Printer<'a, 'b, 's> {
    |          --  -- lifetime `'s` defined here
    |          |
    |          lifetime `'b` defined here
...
990 |                 this.print_sep_list(Self::print_type, ", ")?;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'s`
    |
    = help: consider adding the following bound: `'b: 's`
help: the following changes may resolve your lifetime errors
  |
  |
  = help: `'a` and `'s` must be the same: replace one with the other
  = help: add bound `'b: 's`
error: lifetime may not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/v0.rs:1094:46
     |
     |
632  |   impl<'a, 'b, 's> Printer<'a, 'b, 's> {
     |            --  -- lifetime `'s` defined here
     |            |
     |            lifetime `'b` defined here
...
1094 |           let mut open_brace_if_outside_expr = |this: &mut Self| {
     |  ______________________________________________^
1095 | |             // If this expression is nested in another, braces aren't required.
1096 | |             if in_value {
1097 | |                 return Ok(());
...    |
1101 | |             this.print("{")
1102 | |         };
     | |_________^ assignment requires that `'b` must outlive `'s`
     |
     = help: consider adding the following bound: `'b: 's`
     = note: requirement occurs because of a mutable reference to `Printer<'_, '_, '_>`
     = note: mutable references are invariant over their type parameter
     = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/v0.rs:1135:17
     |
     |
632  | impl<'a, 'b, 's> Printer<'a, 'b, 's> {
     |      --      -- lifetime `'s` defined here
     |      |
     |      lifetime `'a` defined here
...
1135 |                 open_brace_if_outside_expr(self)?;
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'s`
     |
     = help: consider adding the following bound: `'a: 's`
     = note: requirement occurs because of a mutable reference to `Printer<'_, '_, '_>`
     = note: mutable references are invariant over their type parameter
     = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/v0.rs:1135:17
     |
     |
632  | impl<'a, 'b, 's> Printer<'a, 'b, 's> {
     |      --      -- lifetime `'s` defined here
     |      |
     |      lifetime `'a` defined here
...
1135 |                 open_brace_if_outside_expr(self)?;
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'s` must outlive `'a`
     |
     = help: consider adding the following bound: `'s: 'a`
     = note: requirement occurs because of a mutable reference to `Printer<'_, '_, '_>`
     = note: mutable references are invariant over their type parameter
     = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: could not compile `rustc-demangle` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:47
