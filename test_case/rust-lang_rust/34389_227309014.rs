
> use std::iter::FromIterator;
> struct S;
> impl FromIterator<S> for String {
>  fn from_iter<T: IntoIterator<Item=S>>(s: T) -> String {
>    let mut fin = String::new();
>    for S in s {
>      fin.push('a');
>    }
>    fin
>  }
> }
> 