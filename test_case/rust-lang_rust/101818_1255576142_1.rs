rust
#![feature(error_iter)]

use std::error::Error;
use std::io;

pub fn are_any<E: Error + 'static>(err: &(dyn Error + 'static)) -> bool {
    err.sources()
       .map(unwrap_if_io_error)
       .any(|e| e.is::<E>())
}

// hopefully obvious this would need a better name
fn unwrap_if_io_error<'a>(e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
    e.downcast_ref::<io::Error>()
        .and_then(|e| e.get_ref().map(|e| e as _))
        .unwrap_or(e)
    
}
