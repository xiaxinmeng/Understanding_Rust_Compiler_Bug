
use std::path::Path;

trait RR {}
impl RR for () {}

fn foo<'a>(path: &'a Path) -> impl RR + 'static {
    bar(path)
}
fn bar<P: AsRef<Path>>(path: P) -> impl RR + 'static {
    ()
}
