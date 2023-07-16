rust
pub struct NonNull<T: ?Sized> { ... }

impl<T: ?Sized> !Send for NonNull<T> { }
impl<T: ?Sized> !Sync for NonNull<T> { }

impl<'a, T: ?Sized> From<&'a mut T> for NonNull<T> { ... }
impl<'a, T: ?Sized> From<&'a T> for NonNull<T> { ... }

impl<T: Sized> NonNull<T> {
    pub fn empty() -> Self;
}

impl<T: ?Sized> NonNull<T> {
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self;

    pub fn new(ptr: *mut T) -> Option<Self>;

    pub fn as_ptr(self) -> *mut T;

    pub unsafe fn as_ref(&self) -> &T;

    pub unsafe fn as_mut(&mut self) -> &mut T;
}

impl<T: ?Sized> Box<T> {
    pub fn into_raw_non_null(b: Box<T>) -> NonNull<T>;
}

impl<T: ?Sized> Debug for NonNull<T> { ... }
impl<T: ?Sized> Pointer for NonNull<T> { ... }

impl<T: ?Sized> Clone for NonNull<T> { ... }
impl<T: ?Sized> Copy for NonNull<T> { }

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonNull<U>> for NonNull<T> where T: Unsize<U> { }
impl<T: RefUnwindSafe + ?Sized> UnwindSafe for NonNull<T> { }
