rust
use std::result::Result;
use std::result::Result::Err;

pub trait Service {
    type Error;

    fn run() -> Result<(), Self::Error>;
}

enum ErrorWrapper {
    ServiceError(Service::Error)
}

struct ServiceImpl;

struct ServiceImplError;

impl Service for ServiceImpl {
    type Error = ServiceImplError;

    fn run() -> Result<(), Self::Error> {
        Err(ServiceImplError)
    }
}

fn main() -> Result<(), ErrorWrapper> {
    ServiceImpl::run().map_err(|e| ErrorWrapper::ServiceError(e))
}
