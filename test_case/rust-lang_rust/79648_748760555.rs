rust
use futures::future::{join, ready, Future};
use futures::stream::{self, StreamExt};

fn f() -> impl Future<Output = ()> + Send {
    async {
        let a = &();
        join(
            stream::empty().for_each(|_: ()| async { drop(a) }),
            ready(()),
        )
        .await;
    }
}
