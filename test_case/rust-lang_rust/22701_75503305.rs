 rust
trait Resources: std::marker::PhantomFn<Self> {
    type Buffer;
}

type BufferHandle<R: Resources> = (<R as Resources>::Buffer, ());

struct ParamValues<'a, R: Resources> where
    R: Resources,
    R::Buffer: 'a,
{
    pub blocks: &'a mut Vec<BufferHandle<R>>,
}
