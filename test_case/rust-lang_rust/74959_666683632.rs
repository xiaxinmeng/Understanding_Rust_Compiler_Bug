
/usr/local/google/home/richkadel/.cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/src/lib.rs:
.
.
.
  670|       |            __fn_bitflags! {
  671|       |                /// Returns `true` all of the flags in `other` are contained within `self`.
  672|       |                #[inline]
  673|     94|                pub const fn contains(&self, other: $BitFlags) -> bool {
  674|     94|                    (self.bits & other.bits) == other.bits
  675|     94|                }
  ------------------
  | <clap[521a2cb83d8b579f]::app::settings::Flags>::contains:
  |  673|     34|                pub const fn contains(&self, other: $BitFlags) -> bool {
  |  674|     34|                    (self.bits & other.bits) == other.bits
  |  675|     34|                }
  ------------------
  | <clap[521a2cb83d8b579f]::args::settings::Flags>::contains:
  |  673|     60|                pub const fn contains(&self, other: $BitFlags) -> bool {
  |  674|     60|                    (self.bits & other.bits) == other.bits
  |  675|     60|                }
  ------------------
  676|       |            }
  677|       |
  678|       |            /// Inserts the specified flags in-place.
  679|       |            #[inline]
  680|      7|            pub fn insert(&mut self, other: $BitFlags) {
  681|      7|                self.bits |= other.bits;
  682|      7|            }
  ------------------
  | <clap[521a2cb83d8b579f]::app::settings::Flags>::insert:
  |  680|      3|            pub fn insert(&mut self, other: $BitFlags) {
  |  681|      3|                self.bits |= other.bits;
  |  682|      3|            }
  ------------------
  | <clap[521a2cb83d8b579f]::args::settings::Flags>::insert:
  |  680|      4|            pub fn insert(&mut self, other: $BitFlags) {
  |  681|      4|                self.bits |= other.bits;
  |  682|      4|            }
  ------------------
  683|       |
  684|       |            /// Removes the specified flags in-place.
  685|       |            #[inline]
  686|     11|            pub fn remove(&mut self, other: $BitFlags) {
  687|     11|                self.bits &= !other.bits;
  688|     11|            }
  ------------------
  | <clap[521a2cb83d8b579f]::app::settings::Flags>::remove:
  |  686|      1|            pub fn remove(&mut self, other: $BitFlags) {
  |  687|      1|                self.bits &= !other.bits;
  |  688|      1|            }
  ------------------
  | <clap[521a2cb83d8b579f]::args::settings::Flags>::remove:
  |  686|     10|            pub fn remove(&mut self, other: $BitFlags) {
  |  687|     10|                self.bits &= !other.bits;
  |  688|     10|            }
  ------------------
  689|       |
  690|       |            /// Toggles the specified flags in-place.
  691|       |            #[inline]
  692|      0|            pub fn toggle(&mut self, other: $BitFlags) {
  693|      0|                self.bits ^= other.bits;
  694|      0|            }
  ------------------
