plain
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0599]: no method named `is_empty` found for reference `&Path` in the current scope
     |
     |
1248 |         } else if comps.prefix_verbatim() && !path.is_empty() {
     |                                                    ^^^^^^^^ method not found in `&Path`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
     = note: the following trait defines an item `is_empty`, perhaps you need to implement it:
             candidate #1: `core::iter::ExactSizeIterator`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:01:26
