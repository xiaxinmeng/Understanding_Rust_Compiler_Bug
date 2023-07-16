rust
#[macro_export]
macro_rules! read_line {
    () => {{
        let mut input = String::new();
        match stdin().read_line(&mut input).expect("failed to read stdin") {
            0 => Err(Error::new(
                ErrorKind::UnexpectedEof,
                "input reached eof unexpectedly",
            )),
            _ => Ok(String::from(input.trim_end_matches(&['\n', '\r'][..]))),
        }
    }};
    ($type1:ty) => {{
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read stdin");
        input.trim_end_matches(&['\n', '\r'][..]).parse::<$type1>()
    }};
    ($arg1:expr) => {{
        let stdout = stdout();
        let mut lock = stdout.lock();
        lock.write_all($arg1.as_bytes())
            .expect("failed to write whole buffer");
        lock.flush().expect("failed to flush stdout");
        let mut input = String::new();
        match stdin().read_line(&mut input).expect("failed to read stdin") {
            0 => Err(Error::new(
                ErrorKind::UnexpectedEof,
                "input reached eof unexpectedly",
            )),
            _ => Ok(String::from(input.trim_end_matches(&['\n', '\r'][..]))),
        }
    }};
    ($type1:ty,$arg1:expr) => {{
        let stdout = stdout();
        let mut lock = stdout.lock();
        lock.write_all($arg1.as_bytes())
            .expect("failed to write whole buffer");
        lock.flush().expect("failed to flush stdout");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read stdin");
        input.trim_end_matches(&['\n', '\r'][..]).parse::<$type1>()
    }};
}
