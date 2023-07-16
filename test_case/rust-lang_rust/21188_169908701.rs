 rust
impl Add<Point> for Point { ... }
impl<'a> Add<Point> for &'a Point { ... }
impl<'a> Add<&'a Point> for Point { ... }
impl<'a> Add<&'a Point> for &'a Point { ... }
