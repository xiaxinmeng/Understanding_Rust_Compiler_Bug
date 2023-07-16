rust
trait Shel<T> {
    type Output;
}

struct MyTuple<U, B>(U, B);

impl<U, B, Ur, Br> Shel<MyTuple<Ur, Br>> for MyTuple<U, B>
where
    MyTuple<B, U>: Shel<Self::Output>,
{
    type Output = <MyTuple<B, ()> as Shel<MyTuple<Ur, Br>>>::Output;
}

fn main(){}
