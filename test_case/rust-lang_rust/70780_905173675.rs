rust
fn take_closure<'a, F>(_: F)
where
    F: 'static + FnMut(&'a ())
{
}

pub fn poller_fn<'lua, RetFut>(mut fut: Box<RetFut>,)
where
    RetFut: 'static + FnMut(&'lua()),
{
    // Comment the next line and it compiles
    let mut fut: Box<dyn 'static + FnMut(&'lua())> = fut;
    take_closure(move |_| { fut.as_mut(); });
}
