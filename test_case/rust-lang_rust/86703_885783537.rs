rust
// build-pass
// compile-flags: -Z verbose

#![feature(no_core)]
#[no_core]

pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

pub enum Cow<'a, B: ?Sized + 'a>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

pub struct Rc<T: ?Sized> {
    _t: Box<T>,
}

pub unsafe trait Yokeable<'a>: 'static {
    type Output: 'a;
}

pub struct Yoke<Y: for<'a> Yokeable<'a>, C> {
    // must be the first field for drop order
    // this will have a 'static lifetime parameter, that parameter is a lie
    yokeable: Y,
    cart: C,
}


impl<Y: for<'a> Yokeable<'a>> Yoke<Y, Rc<[u8]>> {
    pub fn project<'this, P>(
        &'this self,
        f: for<'a> fn(&'this <Y as Yokeable<'a>>::Output, &'a ()) -> <P as Yokeable<'a>>::Output,
    ) -> Yoke<P, Rc<[u8]>>
    where
        P: for<'a> Yokeable<'a>,
    {
        unimplemented!()
    }
}

pub fn slice(y: Yoke<&'static str, Rc<[u8]>>) -> Yoke<&'static [u8], Rc<[u8]>> {
    y.project(move |yk, _| yk.as_bytes())
}


unsafe impl<'a, T: 'static + ToOwned + ?Sized> Yokeable<'a> for Cow<'static, T>
where
    <T as ToOwned>::Owned: Sized,
{
    type Output = Cow<'a, T>;

}

unsafe impl<'a, T: 'static + ?Sized> Yokeable<'a> for &'static T {
    type Output = &'a T;
}

fn main() {}
