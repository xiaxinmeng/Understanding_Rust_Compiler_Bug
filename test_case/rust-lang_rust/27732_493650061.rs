rust
pub enum Service<'r, Trait, Impl, Mock>
where
    Trait: ?Sized + 'r,
    Impl: Unsize<Trait> + FromRequest<'a, 'r>,
    Mock: Deref + FromRequest<'a, 'r>,
    <Mock as Deref>::Target: Unsize<Trait>
{
    Prod(Impl, PhantomData<&'r Trait>),
    Test(Mock)
}

pub type MyServiceGuard<'r> = Service<
    'r,
    MyServiceTrait,
    MyServiceImpl<'r>,
    rocket::State<'r, MyServiceMock>
>;
