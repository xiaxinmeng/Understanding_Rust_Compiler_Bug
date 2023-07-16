rust
> fn use_trait<T>(_:T)
> where T: MyTrait<do_some_thing = for <Params, Return:Send> fn(Params)->Return where Params: Send >
> {
> }
> 