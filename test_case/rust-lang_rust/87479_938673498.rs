rust
trait Ownership {
    type Reference<'a, T: 'a>;

    unsafe fn as_ref_pool<'p, T: 'p>(
        pool: &'p AutoreleasePool,
        ptr: *mut T,
    ) -> Self::Reference<'p, T>;
}

enum Owned {}

impl Ownership for Owned {
    type Reference<'a, T: 'a> = &'a mut T;

    unsafe fn as_ref_pool<'p, T: 'p>(
        pool: &'p AutoreleasePool,
        ptr: *mut T,
    ) -> Self::Reference<'p, T> {
        // SAFETY: Bound by function signature (bound to pool)
        // Pointer validity is up to caller
        &mut *ptr
    }
}

enum Shared {}

impl Ownership for Shared {
    type Reference<'a, T: 'a> = &'a T;

    unsafe fn as_ref_pool<'p, T: 'p>(
        pool: &'p AutoreleasePool,
        ptr: *mut T,
    ) -> Self::Reference<'p, T> {
        // SAFETY: Bound by function signature (bound to pool)
        // Pointer validity is up to caller
        &*ptr
    }
}
