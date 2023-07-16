 rust
#![feature(default_type_parameter_fallback)]

enum Maybe<T> { Nothing, Just(T) }
use Maybe::*;

impl<T, U = T> PartialEq<Maybe<U>> for Maybe<T>
    where T: PartialEq<U>
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
    println!("{}", Nothing == Just("foo"));

    println!("{}", Just("foo") == Nothing);
}
