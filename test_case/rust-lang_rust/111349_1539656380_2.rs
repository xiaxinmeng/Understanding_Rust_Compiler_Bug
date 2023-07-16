
error[E0519]: the current crate is indistinguishable from one of its dependencies: it has the same crate-name `thiserror_impl` and was compiled with the same `-C metadata` arguments. This will result in symbol conflicts between the two.                  
   --> /home/adema/.cargo/registry/src/index.crates.io-6f17d22bba15001f/thiserror-1.0.40/src/lib.rs:246:9
    |
246 | pub use thiserror_impl::*;
    |         ^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0519`.
error: could not compile `thiserror` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
error[E0519]: the current crate is indistinguishable from one of its dependencies: it has the same crate-name `zeroize_derive` and was compiled with the same `-C metadata` arguments. This will result in symbol conflicts between the two.
   --> /home/adema/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zeroize-1.6.0/src/lib.rs:246:9
    |
246 | pub use zeroize_derive::{Zeroize, ZeroizeOnDrop};
    |         ^^^^^^^^^^^^^^

error[E0432]: unresolved import `crate::Zeroize`
 --> /home/adema/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zeroize-1.6.0/src/x86.rs:3:43
  |
3 | use crate::{atomic_fence, volatile_write, Zeroize};
  |                                           ^^^^^^^

Some errors have detailed explanations: E0432, E0519.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `zeroize` (lib) due to 2 previous errors
