rust
let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "S");
let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "a", &&(*__self_0_0));
...
::core::fmt::DebugStruct::finish(debug_trait_builder)
