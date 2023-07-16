
// librustc_middle/ty/query/on_disk_cache.rs
         .           macro_rules! encoder_methods {
         .               ($($name:ident($ty:ty);)*) => {
         .                   #[inline]
 2,350,558 ( 0.03%)          $(fn $name(&mut self, value: $ty) -> Result<(), Self::Error> {
13,246,909 ( 0.15%)              self.encoder.$name(value)
 2,350,558 ( 0.03%)          })*
         .               }
         .           }

// librustc_metadata/rmeta/encoder.rs
         .           macro_rules! encoder_methods {
         .               ($($name:ident($ty:ty);)*) => {
 3,146,711 ( 0.03%)          $(fn $name(&mut self, value: $ty) -> Result<(), Self::Error> {
         .                       self.opaque.$name(value)
 2,097,912 ( 0.02%)          })*
         .               }
         .           }

// libserialize/serialize.rs
17,134,437 ( 0.19%)      fn emit_enum_variant<F>(
         .                   &mut self,
         .                   _v_name: &str,
         .                   v_id: usize,
         .                   _len: usize,
         .                   f: F,
         .               ) -> Result<(), Self::Error>
         .               where
         .                   F: FnOnce(&mut Self) -> Result<(), Self::Error>,
         .               {
 3,642,239 ( 0.04%)          self.emit_usize(v_id)?;
   139,606 ( 0.00%)          f(self)
   205,023 ( 0.00%)      }

 1,451,419 ( 0.02%)      fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error>
         .               where
         .                   F: FnOnce(&mut Self) -> Result<(), Self::Error>,
         .               {
 1,034,878 ( 0.01%)          self.emit_usize(len)?;
         .                   f(self)
 1,124,627 ( 0.01%)      }

