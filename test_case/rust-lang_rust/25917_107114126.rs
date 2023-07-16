 rust
trait something
{
    fn do_the_thing(&self) {}
}

impl something for i64 {}

impl something for f64 {}

fn do_something< T :something > ( x :T)  {}

fn main()
{
     do_something( 0 ); //compiler errors out
}
