rust
error: incorrect restriction in `pub`
  --> file.rs:15:1
   |
15 | pub (a) fn afn() {}
   |     ^^^
   |
   = help: valid visibility restrictions are:
           `pub`: visible on every module that imports this module
           `pub(crate)`: visible only on the current crate
           `pub(super)`: visible only in the current module's parent
           `pub(in path::to::module)`: visible only on the specified path
           `pub(self)` or ``: visible only in the current module
help: to make this visible only to module `a`, add `in` before the path:
   | pub (in a) fn afn() {}
