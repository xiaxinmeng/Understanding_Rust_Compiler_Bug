plain

error: const trait implementations may not use non-const default functions
   --> library/core/tests/cmp.rs:225:5
    |
225 | /     impl const PartialEq for S {
226 | |         fn eq(&self, other: &Self) -> bool {
227 | |             self.0 == other.0
229 | |     }
    | |_____^
    |
    = note: `ne` not implemented
    = note: `ne` not implemented

error: const trait implementations may not use non-const default functions
   --> library/core/tests/cmp.rs:231:5
    |
231 | /     impl const PartialOrd for S {
232 | |         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
233 | |             let ret = match (self.0, other.0) {
234 | |                 (a, b) if a > b => Ordering::Greater,
240 | |         }
241 | |     }
    | |_____^
    |
    |
    = note: `lt`, `le`, `gt`, `ge` not implemented
For more information about this error, try `rustc --explain E0635`.
error[E0635]: unknown feature `const_convert`
  --> library/core/tests/lib.rs:11:12
   |
