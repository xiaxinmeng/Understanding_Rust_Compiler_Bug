Rust
#![feature(let_chains)]

struct Pd<T>(std::marker::PhantomData<T>);

impl<T> Pd<T> {
    fn iter(&self) -> Iter<T> {
        todo!()
    }
}

pub struct Iter<'a, T: 'a> {
    inner: Box<dyn IterTrait<'a, T> + 'a>,
}

trait IterTrait<'a, T: 'a> {
    fn clone_box(&self) -> Box<dyn IterTrait<'a, T> + 'a>;
}

fn f(m: Option<Pd<()>>) -> bool {
    if let Some(n) = m
        && let it = n.iter()
        /* other stuff */
    {
        
    }
    
    true
}
