plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.3
    Checking object v0.31.0
    Checking addr2line v0.19.0
error[E0599]: no method named `derived_type` found for reference `&object::pe::ImageSymbol` in the current scope
  --> library/std/src/../../backtrace/src/symbolize/gimli/coff.rs:58:20
   |
58 |             if sym.derived_type() != object::pe::IMAGE_SYM_DTYPE_FUNCTION || section_number == 0 {
   |                    ^^^^^^^^^^^^ method not found in `&ImageSymbol`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
1  | use object::coff::ImageSymbol;


error[E0599]: no method named `name` found for reference `&'a object::pe::ImageSymbol` in the current scope
   --> library/std/src/../../backtrace/src/symbolize/gimli/coff.rs:102:27
    |
102 |         self.symbols[i].1.name(self.strings).ok()
    |                           ^^^^ field, not a method
    = help: items from traits can only be used if the trait is in scope
help: remove the arguments
    |
    |
102 -         self.symbols[i].1.name(self.strings).ok()
102 +         self.symbols[i].1.name.ok()
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
    |
1   | use object::coff::ImageSymbol;

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:12
