
$ cargo expand
pub mod symbol {
    //! An "interner" is a data structure that associates values with usize tags and
    //! allows bidirectional lookup; i.e., given a value, one can easily find the
    //! type, and vice versa.
    use rustc_arena::DroplessArena;
    use rustc_data_structures::fx::FxHashMap;
    use rustc_data_structures::stable_hasher::{HashStable, StableHasher, ToStableHashKey};
    use rustc_macros::HashStable_Generic;
    use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
    use std::cmp::{Ord, PartialEq, PartialOrd};
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::str;
    use crate::{Edition, Span, DUMMY_SP, SESSION_GLOBALS};
    const SYMBOL_DIGITS_BASE: u32 = 1211u32;
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    mod kw_generated {
        // ...
