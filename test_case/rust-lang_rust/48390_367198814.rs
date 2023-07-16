
fn foo1<R: Read + ?Sized>(r: &mut R) { }
fn foo2<R: Read>(r: R) { }

fn bar<R: Read + ?Sized>(r: &mut R) { foo1(r); foo1(r); } // okay
fn bar<R: Read + ?Sized>(r: &mut R) { foo2(r); foo2(r); } // Error: `r` was moved
