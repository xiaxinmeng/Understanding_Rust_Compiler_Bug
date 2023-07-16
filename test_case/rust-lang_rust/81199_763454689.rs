
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:43:32: could not fully normalize `<T as Pointee>::Metadata` 

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `<T as Pointee>::Metadata`
#1 [check_mod_unstable_api_usage] checking for unstable API usage in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
