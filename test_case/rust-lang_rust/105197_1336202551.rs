
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

use std::future::Future;
use std::marker::PhantomData;

trait Lockable<K, V> {
    async fn lock_all_entries(
        &self,
    ) -> impl Future<Output = Guard<'_>>;
}

struct Guard<'a>(PhantomData<&'a ()>);
