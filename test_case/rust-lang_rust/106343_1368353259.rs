plain
   --> library/core/src/slice/iter/macros.rs:219:31
    |
41  | /  macro_rules! iterator {
42  | |      (
43  | |          struct $name:ident -> $ptr:ty,
44  | |          $elem:ty,
...   |
219 | |                          idx = i.unchecked_add(1);
    | |                                ^ help: a local variable with a similar name exists: `f`
421 | |      }
422 | |  }
    | |__- in this expansion of `iterator!`
    |
    |
   ::: library/core/src/slice/iter.rs:133:1
    |
133 |  / iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
134 |  |     fn is_sorted_by<F>(self, mut compare: F) -> bool
135 |  |     where
136 |  |         Self: Sized,
142 |  |     }
143 |  | }}
    |  |__- in this macro invocation


error[E0425]: cannot find value `i` in this scope
   --> library/core/src/slice/iter/macros.rs:219:31
    |
41  | / macro_rules! iterator {
42  | |     (
43  | |         struct $name:ident -> $ptr:ty,
44  | |         $elem:ty,
...   |
219 | |                         idx = i.unchecked_add(1);
    | |                               ^ help: a local variable with a similar name exists: `f`
421 | |     }
422 | | }
    | |_- in this expansion of `iterator!`
    |
    |
   ::: library/core/src/slice/iter.rs:365:1
    |
365 |   iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}

For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] core test:false 6.112
error: could not compile `core` due to 2 previous errors
