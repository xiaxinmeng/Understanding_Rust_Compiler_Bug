rust
extern crate failure;
use failure::Fail;

fn cause_is_io_error(error: &Fail) -> bool {
    match error.cause() {
        Some(cause) => cause.downcast_ref::<std::io::Error>().is_some(),
        None => false,
    }
}
