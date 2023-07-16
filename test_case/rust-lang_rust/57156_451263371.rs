Rust
trait CP<'a, T>: 'a + Fn(T) -> bool {
    fn cb(&self) -> Box<dyn CP<'a, T, Output=bool>>;
}

impl<'a, T, F> CP<'a, T> for F
where F: 'a + Clone + Fn(T) -> bool
{
    fn cb(&self) -> Box<dyn CP<'a, T, Output=bool>> {
        Box::new(self.clone())
    }
}

impl<'a, T> Clone for Box<dyn CP<'a, T, Output=bool>>
    where T: 'a
{
    fn clone(&self) -> Self {
        self.cb()
    }
}

fn main() {
    let x: Box<dyn CP<i32, Output=bool>> = Box::new(move |x| x == 0);
    let y = x.clone();
    println!("{}", x(0));
    println!("{}", y(1));
}
