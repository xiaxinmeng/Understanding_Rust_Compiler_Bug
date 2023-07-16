 rust
fn bar1(g: fn<'a>(x: &'a int) -> &'a int) { g(&3i); }
//           ^~~~
//
// Definition of `bar1` now causes rustc to issue a deprecation warning.

fn bar2(g: for<'a> fn(x: &'a int) -> &'a int) { g(&3i); }
//         ^~~~~~~
//
// You need to use this syntax instead.

// ... BUT ...

fn foo1<'a>(x: &'a int) -> &'a int { x }
//     ^~~~
// This is the only syntax accepted for fn definition forms...

for<'a> fn foo2(x: &'a int) -> &'a int { x } /*
^~~~~~~                                       *
                                              *
...because this syntax is rejected by rustc.  *
                                              */

