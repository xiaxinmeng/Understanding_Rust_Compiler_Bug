
fn bar<'a, Sized? T: 'a + Send + Sync>(f: &T){} // ok

trait MaybeSized {}
fn foo<Sized? T: MaybeSized>(f: &T){}
