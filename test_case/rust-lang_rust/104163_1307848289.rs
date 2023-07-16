plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: implementation has missing stability attribute
  --> library/core/src/iter/sources/once_with.rs:76:1
   |
76 | / impl<F> fmt::Debug for OnceWith<F> {
77 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
78 | |         if self.gen.is_some() {
79 | |             f.write_str("OnceWith(Some)")
83 | |     }
84 | | }
   | |_^


error: implementation has missing stability attribute
  --> library/core/src/iter/sources/repeat_with.rs:80:1
   |
80 | / impl<F> fmt::Debug for RepeatWith<F> {
81 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
82 | |         f.debug_struct("RepeatWith").finish()
84 | | }
   | |_^

error: could not compile `core` due to 2 previous errors
