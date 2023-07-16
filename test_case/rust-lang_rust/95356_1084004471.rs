rust
use std::process::Termination;
fn unwrap_or_exit<T, E: Debug>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("very fatal error:");
            Err::<Infallible, _>(e).report().exit_process()
        }
    }
}
