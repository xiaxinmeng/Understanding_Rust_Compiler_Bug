rust
trait MyTrait {
    async fn do_some_thing(&mut self) -> i32 where fn: Send + Sync + OtherTrait;
}

fn use_trait<F, T>(_: T)
where
    T: MyTrait<fn do_some_thing()=F>,
    F: Send + Sync + OtherTrait
