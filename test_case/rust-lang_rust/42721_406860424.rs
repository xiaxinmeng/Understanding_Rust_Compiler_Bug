rust
#![feature(specialization)]

use std::fmt::{self, Debug};

enum MyOption<T> {
  None,
  Some(T)
}

impl<T> Debug for MyOption<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyOption::None => write!(f, "Unset"),
            MyOption::Some(_) => write!(f, "An unprintable value"),
        }
    }
}

impl<T: Debug> Debug for MyOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyOption::None => write!(f, "Unset"),
            MyOption::Some(t) => t.fmt(f),
        }
    }
}

struct Unprintable;

fn main() {
    println!("{:?}", MyOption::Some(1234));
    println!("{:?}", MyOption::Some(Unprintable));
    println!("{:?}", MyOption::None::<Unprintable>);
}
