rust
// Tagged repr is clever enough to grow tags to fill any padding, e.g.:
// 1.   `T_FF` (one byte of Tag, one byte of padding, two bytes of align=2 Field)
//   -> `TTFF` (Tag has expanded to two bytes, i.e. like `#[repr(u16)]`)
// 2.    `TFF` (one byte of Tag, two bytes of align=1 Field)
//   -> Tag has no room to expand!
//   (this outcome can be forced onto 1. by wrapping Field in `Packed<...>`)
#[repr(packed)]
struct Packed<T>(T);

#[rustc_layout(debug)]
type NicheLosesToTagged = FixedResult<[u64; 0], Packed<std::num::NonZeroU16>>;

#[repr(u16)]
enum U16IsZero { _Zero = 0 }

#[rustc_layout(debug)]
type NicheWinsOverTagged = FixedResult<[u64; 0], Packed<U16IsZero>>;
