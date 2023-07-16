rust
pub struct DoStuff(());

impl std::future::Future for DoStuff {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        todo!()
    }
}

pub fn do_stuff() -> DoStuff {
    DoStuff(())
}
