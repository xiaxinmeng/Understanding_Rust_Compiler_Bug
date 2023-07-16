`rust
#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

pub fn String<V>(elem)
where
    V: 'a,
    for<V> V: IntoIterator,
    for<value> <&str as IntoIterator>::Item: 'a + 'static,
{
        let csv_str: value = String.into_iter().elem::<String>();
}

fn main() {}
