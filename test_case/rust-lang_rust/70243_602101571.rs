rust
/// A stream of PostgreSQL rows that were selected with some properly
/// typed `Selection`.
///
/// Doesn’t actually implement `Stream` because we need to make sure the
/// lifetimes are correct.
pub struct RowStream<S> {
    selection: S,
    stream: postgresql::RowStream,
}

impl<S> RowStream<S> {
    /// Maps the typed borrowed row data into whatever format you want to use.
    /// We don’t implement `Stream` since it’s important that we get the
    /// lifetimes right.
    #[inline]
    pub fn map<X, F, T>(self, mut mapper: F) -> impl Stream<Item = Result<T, Error>>
    where
        X: Table,
        S: for<'a> Selection<'a, X>,
        F: for<'a> FnMut(<S as Selection<'a, X>>::Data) -> T,
    {
        let RowStream { selection, stream } = self;
        stream.map(move |row| {
            let row = row.map_err(log::convert_error!("failed to execute sql query"))?;
            let row = Arc::new(row);
            let data = selection.dangerously_select_from_row(&row);
            let data = mapper(data);
            Ok::<_, Error>(data)
        })
    }

    /// Map all the items and then collect them into a `Vec`. Same as
    /// `stream.map().try_collect::<Vec<_>>()`.
    #[inline]
    pub async fn map_collect<X, F, T>(self, mut mapper: F) -> Result<Vec<T>, Error>
    where
        X: Table,
        S: for<'a> Selection<'a, X>,
        F: for<'a> FnMut(<S as Selection<'a, X>>::Data) -> T,
    {
        self.map(mapper).try_collect().await
    }

    /// Collects all the rows into a `Vec` as long as the data does not
    /// contain references.
    #[inline]
    pub async fn collect<'a, X, T>(self) -> Result<Vec<T>, Error>
    where
        X: Table,
        S: for<'b> Selection<'b, X, Data = T>,
        T: 'static,
    {
        self.map_collect(|row| row).await
    }
}
