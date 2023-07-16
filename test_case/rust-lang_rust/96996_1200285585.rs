rust
type FutureT<'iter, 'future> = impl Future<Output = It<'iter>> /*+ 'future */;
fn get_iter<'iter, 'future>(this: &'iter FutIt, arg: &'future ()) -> FutureT<'iter, 'future> {
    async move {
        let _a = &arg;
        It { _inner: this }
    }
}
