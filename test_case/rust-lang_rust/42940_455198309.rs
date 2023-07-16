rust
fn post<'a, B>(&'a self, _body: &B) -> impl Future + 'a { .. }
