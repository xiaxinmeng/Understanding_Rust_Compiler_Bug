rust
#![feature(error_iter)]

use std::error::Error;
use std::io;

pub fn are_any<E: Error + 'static>(err: &(dyn Error + 'static)) -> bool {
    err.sources()
       .map(|e| {
            e.downcast_ref::<io::Error>()
                .and_then(|e| e.get_ref().map(|e| e as _))
                .unwrap_or(e)
       })
       .any(|e| e.is::<E>())
}
