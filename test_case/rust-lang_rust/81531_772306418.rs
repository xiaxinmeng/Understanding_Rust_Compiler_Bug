rust
use futures::prelude::*;
use failure::bail;

fn main() {
    let _: std::pin::Pin<Box<dyn futures::Future<Output = Result<(), failure::Error>>>> = (async move {
        bail!("")
    }).boxed_local();
}
