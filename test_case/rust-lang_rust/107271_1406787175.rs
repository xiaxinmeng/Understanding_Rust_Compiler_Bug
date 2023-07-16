rust
#![feature(custom_mir, core_intrinsics)]

use core::intrinsics::mir::*;

#[custom_mir(dialect = "built")]
fn drop_term<T>(t: &mut T) {
    mir!(
        {
            Drop(*t, exit)
        }
        
        exit = {
            Return()
        }
    )
}
