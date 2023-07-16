rust

use core::mem::transmute;

struct Scoped<'a>(Option<&'a mut Scoped<'a>>);

unsafe fn scoped<'a: 'b, 'b>(scoped: &'b mut Scoped<'a>) -> Scoped<'b> {
    // This is only safe if we promise to not change the
    // mutable reference that `scoped` holds.
    Scoped(unsafe { transmute(scoped) })
}

// This changes `scoped` to hold a reference to a dropped struct
fn bad(scoped: &mut Scoped<'_>) {
    let mut borrowed_scope = Scoped(None);
    let mut scoped_s = unsafe { scoped(scoped) }; // This shortens the used lifetime.
    scoped_s.0.as_mut().unwrap().0 = Some(&mut borrowed_scope);
}
