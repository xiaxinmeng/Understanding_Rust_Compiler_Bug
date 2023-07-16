rust
trait MiniDataProvider<M>
where
    M: MiniDataMarker
{
    fn mini_load_payload(&self) -> MiniDataPayload<M>;
}

impl<M> MiniDataProvider<M> for MiniStructProvider<M>
where
    M: MiniDataMarker,
    for<'a> <M::Yokeable as MiniYokeable<'a>>::Output: Clone,
{ ... }

let provider = MiniStructProvider { ... };
let payload: MiniDataPayload<SimpleStruct> = provider.mini_load_payload();
