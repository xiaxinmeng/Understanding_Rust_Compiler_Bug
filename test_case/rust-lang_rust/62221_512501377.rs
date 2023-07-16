
pub fn flatten_stream<Fut, St>(future: Fut) -> impl Stream<Item = Fut::Output>
    where Fut: Future<Output = St>,
          St: Stream<Item = Fut::Output>,
{
