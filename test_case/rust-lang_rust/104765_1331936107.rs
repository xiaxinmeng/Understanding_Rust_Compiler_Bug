rust
async fn fail<'a, 'b, 'c>(_: &'static str) where 'a: 'c, 'b: 'c, {}
async fn pass<'a, 'c, 'b>(_: &'static str) where 'a: 'c, 'b: 'c, {}
