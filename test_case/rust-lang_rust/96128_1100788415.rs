
async fn write_items<'a, T, P>(p: &P, items: T) -> Result<(), ()>
    where P: Provider,
        T: IntoIterator<Item=&'a Item>
