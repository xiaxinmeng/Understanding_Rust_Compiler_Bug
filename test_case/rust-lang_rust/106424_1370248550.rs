text
error[E0275]: overflow evaluating the requirement `md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>: md5::digest::typenum::Unsigned`
  |
  = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`rustc_span`)
  = note: required for `UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>, ...>` to implement `std::ops::Shl<md5::digest::typenum::B1>`
  = note: the full type name has been written to 'rustc_span.long-type-8000184375909394518.txt'
  = note: 121 redundant requirements hidden
  = note: required for `md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UTerm, md5::digest::typenum::B1>, md5::digest::typenum::B0>` to implement `std::ops::Shl<md5::digest::typenum::UInt<_, _>>`
  = note: required for `md5::digest::typenum::UTerm` to implement `md5::digest::typenum::uint::SetBit<md5::digest::typenum::UInt<_, _>, md5::digest::typenum::B1>`
  = note: required for `()` to implement `md5::digest::typenum::private::PrivateDivIf<md5::digest::typenum::UInt<Ul, Bl>, md5::digest::typenum::UInt<Ur, Br>, md5::digest::typenum::UTerm, _, md5::digest::typenum::UInt<_, _>, md5::digest::typenum::Greater>`

For more information about this error, try `rustc --explain E0275`.
error: could not document `rustc_span`
