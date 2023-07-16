rust
pub trait Stream {
    type Item;
}

pub trait Future {
    type Output;
}

struct MyStream<T> {
    v: Vec<T>,
}

impl<T> Stream for MyStream<T> {
    type Item = T;
}

pub fn flatten_stream<Fut, St>(future: Fut) -> impl Stream<Item = Fut::Output>
    where Fut: Future<Output = St>,
          St: Stream<Item = Fut::Output>,
{
    MyStream {
        v: Vec::new(),
    }
}
