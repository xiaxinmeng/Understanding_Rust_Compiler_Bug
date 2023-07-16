rust
pub fn join_all<I>(i: I) -> JoinAll<<I as IntoIterator>::Item> where
    I: IntoIterator,
    <I as IntoIterator>::Item: Future, 
