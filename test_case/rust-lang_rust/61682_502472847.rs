rust
enum Option<T> {
    None,
    Some(T),
}

type OptAlias<T> = Option<T>;

fn work_on_alias(x: Option<u8>) -> u8 {
    match x {
        OptAlias::Some(y) => y + 1,
        OptAlias::None => 0,
    }
}
