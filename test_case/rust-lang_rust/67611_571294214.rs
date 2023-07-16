rust
#![crate_type="lib"]

use std::future::Future;
use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub fn timeout<T>(_duration: Duration, _future: T) -> Timeout<T>
where
    T: Future,
{
    loop { }
}

#[derive(Debug)]
pub struct Elapsed(());

impl std::fmt::Display for Elapsed {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(w, "Elapsed")
    }
}

impl std::error::Error for Elapsed {}

impl From<Elapsed> for std::io::Error {
    fn from(_err: Elapsed) -> std::io::Error {
        std::io::ErrorKind::TimedOut.into()
    }
}

pub struct Timeout<T> {
    _value: T,
}

impl<T> Future for Timeout<T>
where
    T: Future,
{
    type Output = std::result::Result<T::Output, Elapsed>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        loop { }
    }
}

static mut TIMEOUT: u64 = 2000;

use std::marker::PhantomData;

pub struct JoinHandle<T> {
    _p: PhantomData<T>,
}

pub fn spawn<T>(_task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    loop { }
}

pub async fn my_main() {
    spawn(proxy());
}

// This version of `fn proxy` demonstrates the error
pub async fn proxy() -> Result<Vec<u8>> {
    timeout
        (Duration::from_millis(unsafe { TIMEOUT }),
         async { Result::Ok(vec![0u8]) })
        .await?
        .unwrap()
        ;

    Result::Ok(vec![0u8])
}

// This variant does not generate an error
pub async fn proxy_2() -> Result<Vec<u8>> {
    let dur = Duration::from_millis(unsafe { TIMEOUT });

    timeout
        (dur,
         async { Result::Ok(vec![0u8]) })
        .await?
        .unwrap()
        ;

    Result::Ok(vec![0u8])
}

// This variant does not generate an error
pub async fn proxy_3() -> Result<Vec<u8>> {
    let tv = unsafe { TIMEOUT };

    timeout
        (Duration::from_millis(tv),
         async { Result::Ok(vec![0u8]) })
        .await?
        .unwrap()
        ;

    Result::Ok(vec![0u8])
}
