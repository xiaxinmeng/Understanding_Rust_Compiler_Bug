plain
   Compiling object v0.30.1
   Compiling hashbrown v0.13.1
   Compiling miniz_oxide v0.6.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
error: Re-exporting `#[doc(hidden)]` item
  --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/miniz_oxide-0.6.2/src/lib.rs:35:1
   |
35 | pub use crate::shared::update_adler32 as mz_adler32_oxide;


error: Re-exporting `#[doc(hidden)]` item
  --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/miniz_oxide-0.6.2/src/lib.rs:36:25
   |
36 | pub use crate::shared::{MZ_ADLER32_INIT, MZ_DEFAULT_WINDOW_BITS};


error: Re-exporting `#[doc(hidden)]` item
  --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/miniz_oxide-0.6.2/src/lib.rs:36:42
   |
36 | pub use crate::shared::{MZ_ADLER32_INIT, MZ_DEFAULT_WINDOW_BITS};

error: could not compile `miniz_oxide` (lib) due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: Re-exporting `#[doc(hidden)]` item
  --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/gimli-0.27.2/src/lib.rs:65:64
   |
65 | pub use crate::endianity::{BigEndian, Endianity, LittleEndian, NativeEndian, RunTimeEndian};

error: could not compile `gimli` (lib) due to previous error
Build completed unsuccessfully in 0:03:56
