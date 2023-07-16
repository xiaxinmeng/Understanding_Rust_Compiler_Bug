rust
pub trait ElementLike {}

pub struct Located<T> where T: ElementLike {
    inner: T,
}

pub struct BlockElement<'a>(&'a str);

impl ElementLike for BlockElement<'_> {}


pub struct Page<'a> {
    /// Comprised of the elements within a page
    pub elements: Vec<Located<BlockElement<'a>>>,
}

impl<'a, __IdxT> ::core::ops::Index<__IdxT> for Page<'a> where
    Vec<Located<BlockElement<'a>>>: ::core::ops::Index<__IdxT>
{
    type Output =
        <Vec<Located<BlockElement<'a>>> as
        ::core::ops::Index<__IdxT>>::Output;

    #[inline]
    fn index(&self, idx: __IdxT) -> &Self::Output {
        <Vec<Located<BlockElement<'a>>> as
                ::core::ops::Index<__IdxT>>::index(&self.elements, idx)
    }
}

fn main() {}
