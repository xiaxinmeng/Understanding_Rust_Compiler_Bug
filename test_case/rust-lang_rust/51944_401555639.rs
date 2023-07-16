rust
#[unstable(feature = "futures_api", issue = "50547")]
impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for PinBox<F> {
    fn into(self) -> FutureObj<T> {
        FutureObj::new(self)
    }
}

#[unstable(feature = "futures_api", issue = "50547")]
impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for Box<F> {
    fn into(self) -> FutureObj<T> {
        FutureObj::new(PinBox::from(self))
    }
}

#[unstable(feature = "futures_api", issue = "50547")]
impl<T, F: Future<Output = T> + 'static> Into<LocalFutureObj<T>> for PinBox<F> {
    fn into(self) -> LocalFutureObj<T> {
        LocalFutureObj::new(self)
    }
}

#[unstable(feature = "futures_api", issue = "50547")]
impl<T, F: Future<Output = T> + 'static> Into<LocalFutureObj<T>> for Box<F> {
    fn into(self) -> LocalFutureObj<T> {
        LocalFutureObj::new(PinBox::from(self))
    }
}
