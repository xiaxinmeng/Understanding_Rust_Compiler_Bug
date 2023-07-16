
#0  0x00005555555ec38c in __rust_probestack ()
#1  0x0000555555560f9a in core::array::try_collect_into_array<core::iter::adapters::map::Map<&mut core::iter::adapters::map::Map<core::array::iter::IntoIter<(), 1000>, core::array::from_fn::{closure_env#0}<[usize; 256], 1000, rust::main::{closure_env#1}>>, fn([usize; 256]) -> core::ops::try_trait::NeverShortCircuit<[usize; 256]>>, [usize; 256], core::ops::try_trait::NeverShortCircuitResidual, 1000> (
    iter=<error reading variable: Cannot access memory at address 0x800000bb7d60>)
    at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/array/mod.rs:793
#2  0x0000555555561ff9 in core::array::try_collect_into_array_unchecked<core::iter::adapters::map::Map<&mut core::iter::adapters::map::Map<core::array::iter::IntoIter<(), 1000>, core::array::from_fn::{closure_env#0}<[usize; 256], 1000, rust::main::{closure_env#1}>>, fn([usize; 256]) -> core::ops::try_trait::NeverShortCircuit<[usize; 256]>>, [usize; 256], core::ops::try_trait::NeverShortCircuitResidual, 1000>
    (iter=0x7fffffa204a8)
    at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/array/mod.rs:764
#3  0x0000555555561f53 in core::array::collect_into_array_unchecked<core::iter::adapters::map::Map<core::array::iter::IntoIter<(), 1000>, core::array::from_fn::{closure_env#0}<[usize; 256], 1000, rust::main::{closure_env#1}>>, 1000> (iter=0x7fffffc144f8)
    at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/array/mod.rs:777
#4  0x0000555555562242 in core::array::{impl#23}::map<(), 1000, core::array::from_fn::{closure_env#0}<[usize; 256], 1000, rust::main::{closure_env#1}>, [usize; 256]> (self=..., f=...)
    at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/array/mod.rs:445
#5  0x000055555556258c in core::array::from_fn<[usize; 256], 1000, rust::main::{closure_env#1}> (
    cb=...) at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/array/mod.rs:44
#6  0x000055555556b9ed in rust::main () at src/main.rs:30
