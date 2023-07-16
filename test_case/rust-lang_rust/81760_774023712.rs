rust
let mut debug_trait_builder = ::core::fmt::Formatter::debug_struct(f, "S");
let _ = ::core::fmt::DebugStruct::field(&mut debug_trait_builder, "a", &&(*__self_0_0));
...
::core::fmt::DebugStruct::finish(&mut debug_trait_builder)
