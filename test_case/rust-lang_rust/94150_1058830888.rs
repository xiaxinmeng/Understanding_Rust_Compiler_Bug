
% git range-diff 9c4378918d0^^..9c4378918d0 a424f42e97f^^..a424f42e97f
1:  87943cd3a31 ! 1:  aa763fcf421 rustdoc-json: Include GenericParamDefKind::Type::synthetic in JSON
    @@ src/rustdoc-json-types/lib.rs
      use serde::{Deserialize, Serialize};
      
      /// rustdoc format-version.
    --pub const FORMAT_VERSION: u32 = 11;
    -+pub const FORMAT_VERSION: u32 = 12;
    +-pub const FORMAT_VERSION: u32 = 12;
    ++pub const FORMAT_VERSION: u32 = 13;
      
      /// A `Crate` is the root of the emitted JSON blob. It contains all type/documentation information
      /// about the language items in the local crate, as well as info about external items to allow
2:  9c4378918d0 = 2:  a424f42e97f rustdoc-json: Make the `fns/generics.rs` test much more robust
