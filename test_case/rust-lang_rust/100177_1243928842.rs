rust
trait Channel<Item>: Sink<Item> + TryStream<Ok = Item> {}

impl<Item, S> Channel<Item> for S where S: Sink<Item> + TryStream<Ok = Item> {}
