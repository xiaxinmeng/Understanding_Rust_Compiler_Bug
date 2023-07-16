
where
    T: GetType<"hello"> + 'static,
    <T as GetType<"hello">>::Ty: 'static,
