rust
pub fn map<X, F, T, U>(self, mut mapper: F) -> impl Stream<Item = Result<U, Error>>
where
    X: Table,
    S: for<'a> Selection<'a, X, Data = T>,
    F: FnMut(T) -> U,
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
