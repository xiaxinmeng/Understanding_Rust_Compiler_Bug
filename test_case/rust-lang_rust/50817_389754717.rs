rust
fn wot(x: impl Iterator<Item = u32> + Send + 'static)
    -> Box<Iterator<Item = u32> + Send>
{ ... }
