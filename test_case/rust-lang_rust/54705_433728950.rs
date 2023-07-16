rust
pub trait ScopeHandle<'scope> {}

pub struct ScopeFutureContents<'scope, S>
    where S: ScopeHandle<'scope>,
{
    dummy: &'scope S,
    this: Box<ScopeFuture<'scope, S>>,
}


struct ScopeFuture<'scope, S>
    where S: ScopeHandle<'scope>,
{
    contents: ScopeFutureContents<'scope, S>,
}

unsafe impl<'scope, S> Send for ScopeFuture<'scope, S>
    where S: ScopeHandle<'scope>,
{}
unsafe impl<'scope, S> Sync for ScopeFuture<'scope, S>
    where S: ScopeHandle<'scope>,
{}
