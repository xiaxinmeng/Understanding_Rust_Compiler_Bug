
error: reached the recursion limit while instantiating `A::matches::<[closure@src/test/compile-fail/issue-22638.rs:50:23: 52:10 base:&&D, f:&&[closure@src/test/compile-fail/issue-22638.rs:37:33: 37:38]]>`
  --> src/test/compile-fail/issue-22638.rs:20:5
   |
20 |       pub fn matches<F: Fn()>(&self, f: &F) {
   |  _____^ starting here...
21 | |         //~^ ERROR reached the recursion limit while instantiating `A::matches::<[closure
22 | |         let &A(ref term) = self;
23 | |         term.matches(f);
24 | |     }
   | |_____^ ...ending here
