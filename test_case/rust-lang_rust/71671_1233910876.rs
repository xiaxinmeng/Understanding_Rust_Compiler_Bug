rust
/*
error: implementation of `Iterator` is not general enough
  --> src/platform/mod.rs:13:9
   |
13 |         tokio::spawn(Thing::task(thing.clone()));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Iterator` is not general enough
   |
   = note: `Iterator` would have to be implemented for the type `std::slice::Iter<'0, i32>`, for any lifetime `'0`...
   = note: ...but `Iterator` is actually implemented for the type `std::slice::Iter<'1, i32>`, for some specific lifetime `'1`

error: implementation of `FnOnce` is not general enough
  --> src/platform/mod.rs:13:9
   |
13 |         tokio::spawn(Thing::task(thing.clone()));
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'0 i32) -> impl futures::Future<Output = ()>` must implement `FnOnce<(&i32,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(&i32,)>`
*/

use std::{sync::Arc, time::Duration};

use futures::{future::join_all, StreamExt};
struct Thing{}

impl Thing {
    pub async fn new() -> Arc<Self> {
        let thing = Arc::new(Thing{});
        tokio::spawn(Thing::task(thing.clone()));
        //Thing::task(thing.clone()).await; //This compiles
        thing
    }

    pub async fn task(self : Arc<Self>) {
        self.task_inner().await;
    }

    pub async fn task_inner(&self){
        let v = vec![1,2,3];
        
        let futs = v.iter() // make this into_iter() and it works
            .map(|a| async move {});
        
        //uncomment this and it works
        //let futs : Vec<_> = futs.collect();
        
        //always works:
        //join_all(futs).await;

        //results in error all the way in new()
        let stream = futures::stream::iter(futs).buffer_unordered(10);
        let results = stream.collect::<()>().await;
    }
}
