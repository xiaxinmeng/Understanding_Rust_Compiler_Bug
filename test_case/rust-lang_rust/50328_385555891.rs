
Building stage2 tool miri (x86_64-apple-darwin)
   Compiling miri v0.1.0 (file:///Users/alex/Software/rust/src/tools/miri)
   Compiling byteorder v1.2.2
error[E0432]: unresolved import `syntax::abi`
 --> tools/miri/miri/fn_call.rs:7:13
  |
7 | use syntax::abi::Abi;
  |             ^^^ Could not find `abi` in `syntax`

error[E0308]: match arms have incompatible types
   --> tools/miri/miri/fn_call.rs:178:25
    |
178 |           let link_name = match attr::first_attr_value_str_by_name(&attrs, "link_name") {
    |  _________________________^
179 | |             Some(name) => name.as_str(),
180 | |             None => self.tcx.item_name(def_id),
    | |                     -------------------------- match arm with an incompatible type
181 | |         };
    | |_________^ expected struct `syntax::symbol::LocalInternedString`, found struct `syntax::symbol::InternedString`
    |
    = note: expected type `syntax::symbol::LocalInternedString`
               found type `syntax::symbol::InternedString`

error[E0608]: cannot index into a value of type `syntax::symbol::InternedString`
  --> tools/miri/miri/intrinsic.rs:32:31
   |
32 |         let intrinsic_name = &self.tcx.item_name(instance.def_id())[..];
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `to_ty` found for type `rustc::ty::layout::Primitive` in the current scope
   --> tools/miri/miri/validation.rs:461:47
    |
461 |                         return Ok(discr.value.to_ty(tcx))
    |                                               ^^^^^
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
            candidate #1: `use rustc::ty::layout::PrimitiveExt;`

error: aborting due to 4 previous errors

Some errors occurred: E0308, E0432, E0599, E0608.
For more information about an error, try `rustc --explain E0308`.
error: Could not compile `miri`.
