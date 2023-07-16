
  677|       |
  678|       |            /// Inserts the specified flags in-place.
  679|       |            #[inline]
  ------------------
  | <clap::app::settings::Flags>::insert:
  |  680|      3|            pub fn insert(&mut self, other: $BitFlags) {
  ------------------
  681|       |                self.bits |= other.bits;
  682|       |            }
  683|       |
  684|       |            /// Removes the specified flags in-place.
  685|       |            #[inline]
  686|     11|            pub fn remove(&mut self, other: $BitFlags) {
  ------------------
  | <clap::app::settings::Flags>::remove:
  |  686|      1|            pub fn remove(&mut self, other: $BitFlags) {
  ------------------
  | <clap::args::settings::Flags>::remove:
  |  686|     10|            pub fn remove(&mut self, other: $BitFlags) {
  ------------------
  687|       |                self.bits &= !other.bits;
  688|       |            }
