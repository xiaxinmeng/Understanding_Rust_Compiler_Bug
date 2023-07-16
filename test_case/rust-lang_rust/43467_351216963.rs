rust
pub struct CFRef<T>(*const CFShared<T>);
pub struct CFShared<T>(T);
