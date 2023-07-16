
> trait DerefMove<E> : DerefImm<E> {
>     fn deref_copy(&self) -> E;
> }
> 