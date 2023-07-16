rust
use futures_util::compat::Future01CompatExt;
use gluon::{
    vm::api::{FunctionRef, Getable, Pushable, VmType},
};

pub type Error = gluon::Error;

pub struct Script<'banana, 'peach, In, Out>
where
    In: VmType + Pushable<'banana>,
    Out: VmType + Getable<'banana, 'peach> + Send + Sync,
{
    vm: gluon::RootedThread,
}

impl<'banana, 'peach, In, Out> Script<'banana, 'peach, In, Out>
where
    In: VmType + Pushable<'banana>,
    Out: VmType + Getable<'banana, 'peach> + Send + Sync,
{
    pub async fn run(&mut self, input: &In) -> Result<Out, Error> {
        let func: FunctionRef<fn(In) -> Out> = self.vm.get_global("main")?;

        return func.call_async(input).compat().await;
    }
}
