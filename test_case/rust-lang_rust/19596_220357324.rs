
error: overflow evaluating the requirement `[closure@src/librustc/hir/pat_util.rs:177:14: 184:6 dm:&'static &'static std::collections::HashMap<u32, hir::def::PathResolution, core::hash::BuildHasherDefault<rustc_data_structures::fnv::FnvHasher>>, contains_bindings:&'static mut bool]: core::ops::FnMut<(&'static hir::Pat,)>` [E0275]
src/librustc/lib.rs:1:1: 1:1 note: consider adding a `#![recursion_limit="128"]` attribute to your crate
error: aborting due to previous error
