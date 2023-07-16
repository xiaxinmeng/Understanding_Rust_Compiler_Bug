rust
fn read_line<T: std::fmt::Display + std::fmt::Debug>(
    prompt: &str,
) -> std::result::Result<T, <T as std::str::FromStr>::Err>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdout = stdout();
    let mut lock = stdout.lock();
    lock.write_all(prompt.as_bytes())
        .expect("failed to write whole buffer");
    lock.flush().expect("failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read stdin");
    input.trim_end_matches(&['\n', '\r'][..]).parse::<T>()
}
