rust
macro_rules! mac {
    ($[i=index, l=length]($word:ident),*) => {
        $(println!("{}/{}: {}", $i, $l, stringify!($word));)*
    };
}
