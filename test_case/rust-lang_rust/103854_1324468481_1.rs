`rust
fn use_trait<T, U, Params, Return>(_:T)
where T: MyTrait< Output = U, do_some_thing = fn(Params)->Return >  // consistent
Params: Send,
Return: Send
{
}
