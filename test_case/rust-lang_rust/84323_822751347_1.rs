
  677|       |
  678|       |            /// Inserts the specified flags in-place.
  679|       |            #[inline]
  680|      7|            pub fn insert(&mut self, other: $BitFlags) {
  681|      7|                self.bits |= other.bits;
  682|      7|            }
  ------------------
  | <clap::args::settings::Flags>::insert:
  |  680|      4|            pub fn insert(&mut self, other: $BitFlags) {
  |  681|      4|                self.bits |= other.bits;
  |  682|      4|            }
  ------------------
  | <clap::app::settings::Flags>::insert:
  |  680|      3|            pub fn insert(&mut self, other: $BitFlags) {
  |  681|      3|                self.bits |= other.bits;
  |  682|      3|            }
  ------------------
  683|       |
  684|       |            /// Removes the specified flags in-place.
  685|       |            #[inline]
  686|     11|            pub fn remove(&mut self, other: $BitFlags) {
  687|     11|                self.bits &= !other.bits;
  688|     11|            }
  ------------------
  | <clap::args::settings::Flags>::remove:
  |  686|     10|            pub fn remove(&mut self, other: $BitFlags) {
  |  687|     10|                self.bits &= !other.bits;
  |  688|     10|            }
  ------------------
  | <clap::app::settings::Flags>::remove:
  |  686|      1|            pub fn remove(&mut self, other: $BitFlags) {
  |  687|      1|                self.bits &= !other.bits;
  |  688|      1|            }
