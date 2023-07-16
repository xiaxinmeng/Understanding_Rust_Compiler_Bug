rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_backtrace_fmt() {
        let bt = Backtrace::capture();
        eprintln!("uncaptured: {:?}", bt);
        let bt = Backtrace::force_capture();
        eprintln!("captured: {:?}", bt);
        eprintln!("display print: {}", bt);
        eprintln!("resolved: {:?}", bt);
        unimplemented!();
    }
}
