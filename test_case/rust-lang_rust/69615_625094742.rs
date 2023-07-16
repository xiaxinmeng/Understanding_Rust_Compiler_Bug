rust
> > #![feature(const_trait_impl, const_fn, const_if_match)]
> > 
> > pub trait MyTrait {
> >     fn method(&self);
> > }
> > 
> > impl const MyTrait for std::convert::Infallible {
> >     #[inline(always)]
> >     fn method(&self) {
> >         while let Some(_) = Option::<()>::None {}
> >     }
> > }
> > 