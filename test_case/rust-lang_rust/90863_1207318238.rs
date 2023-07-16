
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<std::convert::Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>
   = note: required because of the requirements on the impl of `From<bytemuck::PodCastError>` for `anyhow::Error`
   = note: required because of the requirements on the impl of `FromResidual<Result<std::convert::Infallible, bytemuck::PodCastError>>` for `Result<(), anyhow::Error>`
