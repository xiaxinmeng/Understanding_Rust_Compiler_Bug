rust
> impl<T: Send + Sync> DataPtrConvert<T> for Arc<T> {
>     type Weak = sync::sync::Weak<T>;
> 
>     fn as_weak(&self) -> Self::Weak { todo!() }
> }
> 