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
error[E0635]: unknown feature `const_num_from_num`
  --> library/core/tests/lib.rs:16:12
   |
16 | #![feature(const_num_from_num)]
