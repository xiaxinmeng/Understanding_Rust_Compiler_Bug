rs
#[repr(transparent)]
struct DynSendWrapper(dyn Send);

pub fn test() -> impl Send {
    async { bar::<Box<DynSendWrapper>>().await }
}
