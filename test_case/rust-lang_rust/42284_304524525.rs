
struct A;
struct B(());

trait MyIntoFuture {
    type Item;
    type Error;
}

trait MyStream {
    type Item;
    type Error;
    fn then<F>(self, f: F) -> ()
        where F: FnMut(Result<Self::Item, Self::Error>) -> (),
              Self: Sized;
}

struct MyBufferUnordered<S>(S)
    where S: MyStream, S::Item: MyIntoFuture;

impl<S> MyStream for MyBufferUnordered<S>
        where S: MyStream, S::Item: MyIntoFuture<Error=S::Error> {

    type Item = <S::Item as MyIntoFuture>::Item;
    type Error = S::Error;

    fn then<F>(self, _: F) -> ()
            where F: FnMut(Result<Self::Item, Self::Error>) -> (), Self: Sized {
        panic!()
    }
}

impl<F: ?Sized + MyIntoFuture> MyIntoFuture for ::std::boxed::Box<F> {
    type Item = F::Item;
    type Error = F::Error;
}
impl<S: ?Sized + MyStream> MyStream for ::std::boxed::Box<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn then<F>(self, _: F) -> ()
            where F: FnMut(Result<Self::Item, Self::Error>) -> (), Self: Sized {
        panic!()
    }
}

fn main() {
    let b: MyBufferUnordered<Box<MyStream<Item=Box<MyIntoFuture<Item=A, Error=B>>, Error=B>>> = panic!();
    //b.then(|res              | -> () { // works
    //b.then(|res: Result<_, _>| -> () { // breaks
    //b.then(|res: Result<A, B>| -> () { // works
        if let Err(e) = res {
            e.0;
        }
    });
}
