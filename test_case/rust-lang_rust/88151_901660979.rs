
error[E0433]: failed to resolve: use of undeclared crate or module `object`
 --> library/std/src/../../backtrace/src/symbolize/gimli/macho.rs:4:5
  |
4 | use object::read::macho::{MachHeader, Nlist, Section, Segment as _};
  |     ^^^^^^ use of undeclared crate or module `object`
