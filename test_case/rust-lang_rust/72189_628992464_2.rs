
// librustc_middle/ty/query/on_disk_cache.rs
         .           macro_rules! encoder_methods {
         .               ($($name:ident($ty:ty);)*) => {
         .                   #[inline]
51,983,394 ( 0.55%)          $(fn $name(&mut self, value: $ty) -> Result<(), Self::Error> {
11,988,316 ( 0.13%)              self.encoder.$name(value)
52,284,125 ( 0.55%)          })*
         .               }
         .           }

// librustc_metadata/rmeta/encoder.rs
         .           macro_rules! encoder_methods {
         .               ($($name:ident($ty:ty);)*) => {
47,736,282 ( 0.50%)          $(fn $name(&mut self, value: $ty) -> Result<(), Self::Error> {
         .                       self.opaque.$name(value)
41,334,541 ( 0.43%)          })*
         .               }
         .           }

// libserialize/serialize.rs
76,649,911 ( 0.80%)      fn emit_enum_variant<F>(
         .                   &mut self,
         .                   _v_name: &str,
         .                   v_id: usize,
         .                   _len: usize,
         .                   f: F,
         .               ) -> Result<(), Self::Error>
         .               where
         .                   F: FnOnce(&mut Self) -> Result<(), Self::Error>,
         .               {
 3,668,046 ( 0.04%)          self.emit_usize(v_id)?;
   277,579 ( 0.00%)          f(self)
30,539,486 ( 0.32%)      }

20,015,830 ( 0.21%)      fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error>
         .               where
         .                   F: FnOnce(&mut Self) -> Result<(), Self::Error>,
         .               {
 1,034,878 ( 0.01%)          self.emit_usize(len)?;
         .                   f(self)
16,012,664 ( 0.17%)      }
