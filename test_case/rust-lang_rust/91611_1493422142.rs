
#![feature(return_position_impl_trait_in_trait)]

use std::time::Duration;
use async_trait::async_trait;
use async_stream::stream;
use futures::{Stream, StreamExt, pin_mut};

// #[async_trait]
pub trait Iterator {
    fn iterate(&self) -> impl Stream<Item = usize> + '_;
}

pub struct Implementer {}

impl Implementer {
    fn generate(&self) -> impl Stream<Item = usize> {
        stream! {
            yield 1;
        }
    }
}

// #[async_trait]
impl Iterator for Implementer {
    fn iterate(&self) -> impl Stream<Item = usize> + '_ {
    
        // uncomment the next line to get an error [E0728]
        //tokio::time::sleep(Duration::from_millis(10)).await;

        stream! {
            let inner = self.generate();
            pin_mut!(inner);

            while let Some(i) = inner.next().await {
                yield i;
            }
        }
    }
}
