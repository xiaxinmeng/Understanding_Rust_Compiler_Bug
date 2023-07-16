rust
pub fn my_middleware<N>(route: N)
where
    N: FnOnce() -> (),
{
    log(|| {
        log(|| {
            log(|| {
                log(|| {
                    ...
                    log(route);
                });
            });
        });
    })
}
