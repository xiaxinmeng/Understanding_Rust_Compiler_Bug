
async fn write_items<'a, T, I, P>(p: &P, items: T) -> Result<(), ()>
    where P: Provider,
        I: Iterator<Item=&'a Item> + Send,
        T: IntoIterator<Item=&'a Item, IntoIter=I>
