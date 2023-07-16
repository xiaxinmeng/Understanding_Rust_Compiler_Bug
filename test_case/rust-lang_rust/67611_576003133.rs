rust
auto trait DoesntContainRawMutPtr {}

impl<T: ?Sized> !DoesntContainRawMutPtr for *mut T {}
impl<T: ?Sized> DoesntContainRawMutPtr for *const T {}
impl<T: ?Sized> DoesntContainRawMutPtr for &T {}
impl<T: ?Sized> DoesntContainRawMutPtr for &mut T {}
