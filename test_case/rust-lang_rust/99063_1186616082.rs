plain
    |
862 |         literal,
    |         ^^^^^^^

error[E0432]: unresolved import `super::kw::MacroRules`
     |
     |
1940 |     pub use super::kw::MacroRules as macro_rules;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MacroRules` in `symbol::kw`

error[E0425]: cannot find value `rust_2015_preview` in module `sym`
   |
   |
63 |             Edition::Edition2015 => sym::rust_2015_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`

error[E0425]: cannot find value `rust_2018_preview` in module `sym`
   |
   |
64 |             Edition::Edition2018 => sym::rust_2018_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`

error[E0425]: cannot find value `rust_2021_preview` in module `sym`
   |
   |
65 |             Edition::Edition2021 => sym::rust_2021_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`

error[E0425]: cannot find value `rust_2024_preview` in module `sym`
   |
   |
66 |             Edition::Edition2024 => sym::rust_2024_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`
error[E0425]: cannot find value `include` in module `sym`
   --> compiler/rustc_span/src/hygiene.rs:321:76
    |
    |
321 |                 || expn_data.kind == ExpnKind::Macro(MacroKind::Bang, sym::include)
    |                                                                            ^^^^^^^ not found in `sym`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
377 |                 dollar_crate_name: kw::DollarCrate,
    |                                        ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
552 |                         dollar_crate_name: kw::DollarCrate,
    |                                                ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
572 |                         dollar_crate_name: kw::DollarCrate,
    |                                                ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
588 |                 dollar_crate_name: kw::DollarCrate,
    |                                        ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
611 |                 .take_while(|scdata| scdata.dollar_crate_name == kw::DollarCrate)
    |                                                                      ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `PathRoot` in module `kw`
     |
     |
1062 |             ExpnKind::Root => kw::PathRoot.to_string(),
     |                                   ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
1348 |             dollar_crate_name: kw::Empty,
     |                                    ^^^^^ not found in `kw`
help: consider importing one of these items
     |
27   | use core::num::IntErrorKind::Empty;
     |
     |
27   | use crate::hygiene::field::Empty;
27   | use std::num::IntErrorKind::Empty;
     |
27   | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate
help: if you import `Empty`, refer to it directly
     |
1348 -             dollar_crate_name: kw::Empty,
1348 +             dollar_crate_name: Empty,


error[E0425]: cannot find value `DollarCrate` in module `kw`
     |
     |
1366 |     ctxt_data.dollar_crate_name = kw::DollarCrate;
     |                                       ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
1375 |         assert_eq!(dummy.dollar_crate_name, kw::Empty);
     |                                                 ^^^^^ not found in `kw`
help: consider importing one of these items
     |
27   | use core::num::IntErrorKind::Empty;
     |
     |
27   | use crate::hygiene::field::Empty;
27   | use std::num::IntErrorKind::Empty;
     |
27   | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate
help: if you import `Empty`, refer to it directly
     |
1375 -         assert_eq!(dummy.dollar_crate_name, kw::Empty);
1375 +         assert_eq!(dummy.dollar_crate_name, Empty);


error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
1604 |         Ident::with_dummy_span(kw::Empty)
     |                                    ^^^^^ not found in `kw`
help: consider importing one of these items
     |
5    | use core::num::IntErrorKind::Empty;
     |
     |
5    | use crate::source_map::tracing::field::Empty;
5    | use std::num::IntErrorKind::Empty;
     |
5    | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate
help: if you import `Empty`, refer to it directly
     |
1604 -         Ident::with_dummy_span(kw::Empty)
1604 +         Ident::with_dummy_span(Empty)


error[E0425]: cannot find value `DollarCrate` in module `kw`
     |
     |
1721 |         } else if self.symbol == kw::DollarCrate {
     |                                      ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
1804 |         self == kw::Empty
     |                     ^^^^^ not found in `kw`
help: consider importing one of these items
     |
5    | use core::num::IntErrorKind::Empty;
     |
     |
5    | use crate::source_map::tracing::field::Empty;
5    | use std::num::IntErrorKind::Empty;
     |
5    | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate
help: if you import `Empty`, refer to it directly
     |
1804 -         self == kw::Empty
1804 +         self == Empty


error[E0425]: cannot find value `Underscore` in module `kw`
     |
     |
1958 |         self <= kw::Underscore
     |                     ^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `As` in module `kw`
     |
     |
1962 |         self >= kw::As && self <= kw::While
     |                     ^^ not found in `kw`

error[E0425]: cannot find value `While` in module `kw`
     |
     |
1962 |         self >= kw::As && self <= kw::While
     |                                       ^^^^^ not found in `kw`

error[E0425]: cannot find value `Async` in module `kw`
     |
     |
1966 |         (self >= kw::Async && self <= kw::Dyn) && edition() >= Edition::Edition2018
     |                      ^^^^^ not found in `kw`
help: consider importing this unit variant
     |
5    | use crate::DesugaringKind::Async;
     |
     |
help: if you import `Async`, refer to it directly
     |
1966 -         (self >= kw::Async && self <= kw::Dyn) && edition() >= Edition::Edition2018
1966 +         (self >= Async && self <= kw::Dyn) && edition() >= Edition::Edition2018


error[E0425]: cannot find value `Dyn` in module `kw`
     |
     |
1966 |         (self >= kw::Async && self <= kw::Dyn) && edition() >= Edition::Edition2018
     |                                           ^^^ not found in `kw`

error[E0425]: cannot find value `Abstract` in module `kw`
     |
     |
1970 |         self >= kw::Abstract && self <= kw::Yield
     |                     ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Yield` in module `kw`
     |
     |
1970 |         self >= kw::Abstract && self <= kw::Yield
     |                                             ^^^^^ not found in `kw`

error[E0425]: cannot find value `Try` in module `kw`
     |
     |
1974 |         self == kw::Try && edition() >= Edition::Edition2018
     |                     ^^^ not found in `kw`

error[E0425]: cannot find value `Super` in module `kw`
     |
     |
1987 |         self == kw::Super
     |                     ^^^^^ not found in `kw`

error[E0425]: cannot find value `SelfLower` in module `kw`
     |
     |
1988 |             || self == kw::SelfLower
     |                            ^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `SelfUpper` in module `kw`
     |
     |
1989 |             || self == kw::SelfUpper
     |                            ^^^^^^^^^ not found in `kw`
error[E0425]: cannot find value `Crate` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1990:28
     |
     |
1990 |             || self == kw::Crate
     |                            ^^^^^ not found in `kw`

error[E0425]: cannot find value `PathRoot` in module `kw`
     |
     |
1991 |             || self == kw::PathRoot
     |                            ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
     |
     |
1992 |             || self == kw::DollarCrate
     |                            ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `True` in module `kw`
     |
     |
1997 |         self == kw::True || self == kw::False
     |                     ^^^^ not found in `kw`

error[E0425]: cannot find value `False` in module `kw`
     |
     |
1997 |         self == kw::True || self == kw::False
     |                                         ^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
2002 |         self != kw::Empty && self != kw::Underscore && !self.is_path_segment_keyword()
     |                     ^^^^^ not found in `kw`
help: consider importing one of these items
     |
5    | use core::num::IntErrorKind::Empty;
     |
     |
5    | use crate::source_map::tracing::field::Empty;
5    | use std::num::IntErrorKind::Empty;
     |
5    | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate
help: if you import `Empty`, refer to it directly
     |
2002 -         self != kw::Empty && self != kw::Underscore && !self.is_path_segment_keyword()
2002 +         self != Empty && self != kw::Underscore && !self.is_path_segment_keyword()


error[E0425]: cannot find value `Underscore` in module `kw`
     |
     |
2002 |         self != kw::Empty && self != kw::Underscore && !self.is_path_segment_keyword()
     |                                          ^^^^^^^^^^ not found in `kw`
Some errors have detailed explanations: E0425, E0432.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_span` due to 37 previous errors
warning: build failed, waiting for other jobs to finish...
