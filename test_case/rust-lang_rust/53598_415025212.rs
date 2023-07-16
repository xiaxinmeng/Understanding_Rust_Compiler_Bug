rust
#![feature(existential_type)]

pub trait Engine {}
pub trait RenderImplementation<E, CE> 
    where E: Engine,
          CE: Engine,
{
    fn render_impl<C: Renderable<CE>>(&self, eng: &E, children: &C);
}
pub trait Renderable<E> 
    where E: Engine,
{
    fn render(&self, eng: &E);
}
pub trait View<E, CE>
    where E: Engine,
          CE: Engine,
{
    type Renderable: Renderable<E>;

    fn build<C: Renderable<CE> + 'static>(self, children: Option<C>) -> Self::Renderable;
}

pub struct Node <E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<E, CHE>,
        CH: Renderable<CHE>
{
    _m: std::marker::PhantomData<(I, CH, E, CHE)>
}

impl<E, I, CHE, CH> Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<E, CHE>,
        CH: Renderable<CHE>
{
    pub fn new(_item: I, _children: CH) -> Self {
        Self {
            _m: Default::default()
        }
    }
}

impl<E, I, CHE, CH> Renderable<E> for Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<E, CHE>,
        CH: Renderable<CHE>
{
    fn render(&self, _eng: &E) {}
}

impl <E: Engine, T: Renderable<E>> Renderable<E> for Option<T> {
    fn render(&self, _eng: &E) {}
}

pub struct HtmlEngine;
impl Engine for HtmlEngine {}

pub struct Div;

impl RenderImplementation<HtmlEngine, HtmlEngine> for Div {   
    fn render_impl<C>(&self, _eng: &HtmlEngine, _children: &C)
        where C: Renderable<HtmlEngine> 
    {}
}

impl View<HtmlEngine, HtmlEngine> for Div {
    existential type Renderable: Renderable<HtmlEngine>;

    fn build<C: Renderable<HtmlEngine> + 'static>(self, children: Option<C>) -> Self::Renderable {
        Node::new(self, children)
    }
}

#[derive(Default)]
pub struct Stub<E: Engine> {
    _e: std::marker::PhantomData<E>
}

impl<E: Engine> Renderable<E> for Stub<E>  {
    fn render(&self, _eng: &E) {}
}

fn main() {
    Div.build::<Stub<_>>(None);
}
