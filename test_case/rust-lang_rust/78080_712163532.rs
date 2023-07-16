rust
struct Wrapper<T>(T);
struct Point<T>(T);

trait First<T> { }
trait Second<T> { }

impl<C, T> First<T> for Wrapper<C> where Wrapper<C>: First<Point<T>> { }

impl<C, T> Second<T> for Wrapper<C> where Wrapper<C>: First<T> { }

impl<C, T> Second<T> for Wrapper<C> where Wrapper<C>: First<Point<T>> { }

