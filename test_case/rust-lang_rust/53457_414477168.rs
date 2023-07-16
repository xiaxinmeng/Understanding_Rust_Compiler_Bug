rust
#![feature(existential_type)]

trait Future {
    fn poll(&self, cx: &mut ());
}

trait Write {
    fn poll_close(&self, cx: &mut ());
}

existential type Close<'a, W: Write>: Future + 'a;

fn broken<'a, W: Write>(w: &'a W) -> Close<'a, W> {
    PollFn(move |cx| w.poll_close(cx))
}

fn working<'a, W: Write>(w: &'a W) -> impl Future + 'a {
    PollFn(move |cx| w.poll_close(cx))
}

pub struct PollFn<F: Fn(&mut ())>(F);

impl<F> Future for PollFn<F> where F: Fn(&mut ()) {
    fn poll(&self, cx: &mut ()) {
        (&self.0)(cx)
    }
}
