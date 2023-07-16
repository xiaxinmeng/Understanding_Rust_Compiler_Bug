 rust
#![feature(default_type_parameter_fallback)]

enum Maybe<T> { Nothing, Just(T) }
use Maybe::*;

trait DefaultTo<Ty, Default> {}
impl<T, U=T> DefaultTo<U, T> for () {}

impl<U, T = U> PartialEq<Maybe<U>> for Maybe<T>
    where T: PartialEq<U>,
          (): DefaultTo<U, T>
{
    fn eq(&self, other: &Maybe<U>) -> bool {
        match (self, other) {
            (&Nothing, &Nothing) => true,
            (&Just(ref a), &Just(ref b)) => a == b,
            _ => false,
        }
    }
}

fn main() {
    println!("{}", PartialEq::eq(&Nothing, &Just("foo")));

    println!("{}", PartialEq::eq(&Just("foo"), &Nothing));
}
