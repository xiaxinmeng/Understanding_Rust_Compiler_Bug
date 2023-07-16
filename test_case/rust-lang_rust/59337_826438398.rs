rust
fn test<'a, T>(_x: &'a T) -> impl Future<Output = ()> + 'a {  // works fine
    std::future::ready(())
}

fn test2<'a, T: 'a>(_x: T) -> impl Future<Output = ()> + 'a {  // error
    std::future::ready(())
}

fn test3<T>(_x: T) -> impl Future<Output = ()> {  // error
    std::future::ready(())
}
