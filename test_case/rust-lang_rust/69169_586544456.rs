rust
pub struct Script {
    vm: gluon::RootedThread,
}

impl Script
{
    pub async fn run<In, Out>(&mut self, input: In) -> Result<Out, Box<dyn std::error::Error>>
    where
        In: VmType + for<'banana> Pushable<'banana>,
        Out: VmType + for<'peach, 'mango> Getable<'peach, 'mango> + Send + Sync + 'static,
    {
        let mut func = self.vm.get_global::<FunctionRef<fn(In) -> Out>>("main")?;

        return Ok(func.call_async(input).compat().await?);
    }
}
