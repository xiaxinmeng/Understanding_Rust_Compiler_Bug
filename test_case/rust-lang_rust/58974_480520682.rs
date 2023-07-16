rust
use std::error::Error;

// used to work
fn f<E: Error + 'static>(e: E) -> Box<dyn Error> {
    e.into()
}
