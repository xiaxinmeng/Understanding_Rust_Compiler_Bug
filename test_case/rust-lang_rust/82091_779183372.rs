
error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:154:34
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
154  | |                 self.super_place(place_ref, context, location);
     | |                                  ^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mut mir::PlaceRef<'tcx>`
...    |
929  | |     }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1084 |   make_mir_visitor!(Visitor,);
     |   ---------------------------- in this macro invocation

error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:383:29
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
383  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
...    |
929  | |     }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1084 |   make_mir_visitor!(Visitor,);
     |   ---------------------------- in this macro invocation
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`

error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:390:29
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
390  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
...    |
929  | |     }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1084 |   make_mir_visitor!(Visitor,);
     |   ---------------------------- in this macro invocation
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`

error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:383:29
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
383  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
...    |
929  | |     }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1085 |   make_mir_visitor!(MutVisitor, mut);
     |   ----------------------------------- in this macro invocation
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`

error[E0308]: mismatched types
    --> compiler/rustc_middle/src/mir/visit.rs:390:29
     |
68   | / macro_rules! make_mir_visitor {
69   | |     ($visitor_trait_name:ident, $($mutability:ident)?) => {
70   | |         pub trait $visitor_trait_name<'tcx> {
71   | |             // Override these, and call `self.super_xxx` to revert back to the
...    |
390  | |                             &mut place.as_ref(),
     | |                             ^^^^^^^^^^^^^^^^^^^ expected struct `mir::PlaceRef`, found `&mir::Place<'_>`
...    |
929  | |     }
930  | | }
     | |_- in this expansion of `make_mir_visitor!`
...
1085 |   make_mir_visitor!(MutVisitor, mut);
     |   ----------------------------------- in this macro invocation
     |
     = note: expected mutable reference `&mut mir::PlaceRef<'tcx>`
                found mutable reference `&mut &mir::Place<'_>`

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`
