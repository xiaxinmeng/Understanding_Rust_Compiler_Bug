
once_cell$ cargo +nightly build --target thumbv6m-none-eabi
   Compiling once_cell v1.14.0 (/home/jan/tmp/once_cell)
thread 'rustc' panicked at 'identifier: "metadata_std_required", attr: None, args: FluentArgs([("add_info", String("")), ("crate_name", String("std")), ("locator_triple", String("thumbv6m-none-eabi"))]), errors: [ResolverError(Reference(Variable { id: "current_crate" }))]', compiler/rustc_errors/src/translation.rs:91:17
