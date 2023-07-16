plain
   Compiling hashbrown v0.9.0
   Compiling object v0.22.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error[E0391]: cycle detected when optimizing MIR for `<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/addr2line-0.14.0/src/lib.rs:886:1: 1080:2>::parse_children`
     |
970  | /     fn parse_children(
970  | /     fn parse_children(
971  | |         entries: &mut gimli::EntriesRaw<R>,
972  | |         depth: isize,
973  | |         unit: &gimli::Unit<R>,
978  | |         inlined_depth: usize,
979  | |     ) -> Result<(), Error> {
     | |__________________________^
     |
     |
note: ...which requires optimizing MIR for `<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/addr2line-0.14.0/src/lib.rs:1082:1: 1176:2>::parse`...
     |
1083 | /     fn parse(
1083 | /     fn parse(
1084 | |         dw_die_offset: gimli::UnitOffset<R::Offset>,
1085 | |         entries: &mut gimli::EntriesRaw<R>,
1086 | |         abbrev: &gimli::Abbreviation,
1093 | |         inlined_depth: usize,
1094 | |     ) -> Result<(), Error> {
     | |__________________________^
     | |__________________________^
     = note: ...which again requires optimizing MIR for `<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/addr2line-0.14.0/src/lib.rs:886:1: 1080:2>::parse_children`, completing the cycle
note: cycle used when optimizing MIR for `<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/addr2line-0.14.0/src/lib.rs:886:1: 1080:2>::parse`
     |
887  | /     fn parse(
887  | /     fn parse(
888  | |         dw_die_offset: gimli::UnitOffset<R::Offset>,
889  | |         unit: &gimli::Unit<R>,
890  | |         sections: &gimli::Dwarf<R>,
891  | |         units: &[ResUnit<R>],
892  | |     ) -> Result<Self, Error> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
