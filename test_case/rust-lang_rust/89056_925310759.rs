rust
trait Future {}
impl Future for () {}

fn sink_error1<'a, F: 'a>(f: F) -> impl Future + 'a {
    sink_error2(f) // error: `F` may not live long enough
}
fn sink_error2<'a, F: 'a>(_: F) -> impl Future + 'a {}
