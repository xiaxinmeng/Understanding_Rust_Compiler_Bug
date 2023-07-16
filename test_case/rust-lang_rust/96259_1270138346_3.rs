
struct A<T> {
    inner: T,
}

fn do_smth() -> Result<(), A<Vec<()>>> {
    Err(()).map_err(|_| {
        let inner = std::iter::once(()).collect();

        // This line triggers ICE
        A { inner }
    })?;

    Ok(())
}
