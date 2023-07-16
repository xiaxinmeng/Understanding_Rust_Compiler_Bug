
macro_rules! todo_iter {
    () => {{
        todo!();
        #[allow(unreachable_code)]
        [].into_iter()
    }};
}

macro_rules! todo_stream {
    () => {{
        todo!();
        struct F<T>(PhantomData<T>);
        impl<T> Stream for F<T> {
            type Item = T;
            fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                Poll::Pending
            }
        }

        #[allow(unreachable_code)]
        F(PhantomData)
    }};
}
