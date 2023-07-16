rust
#[derive(Clone)]
#[repr(transparent)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
    fn wrap(t: &T) -> &Self {
        unsafe { std::mem::transmute(t) }
    }
}

/// Clone requires that the cart derefs to the same address after it is cloned. This works for Rc, Arc, and &'a T.
/// For all other cart types, clone `.baking_cart()` and re-use `attach_to_cart()`.
impl<Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, Rc<T>>
where
    for<'a> Wrap<<Y as Yokeable<'a>>::Output>: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(Wrap::wrap(self.get()).clone().0) },
            cart: self.cart.clone(),
        }
    }
}
