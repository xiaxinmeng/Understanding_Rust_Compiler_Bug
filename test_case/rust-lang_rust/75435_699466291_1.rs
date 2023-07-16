rust
fn input(prompt: &str) -> std::io::Result<String> {
    use std::{io, io::Write};
    if !prompt.is_empty() {
        io::stdout().write_all(prompt.as_bytes())?;
    }
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    s.pop();
    #[cfg(windows)] s.pop();
    Ok(s)
}

trait Items<T,E> {
    fn items(self, f: impl Fn(&str) -> Result<T,E>)
    -> Result<Box<[T]>,E>;
}

impl<T,E> Items<T,E> for &str {
    fn items(self, f: impl Fn(&str) -> Result<T,E>)
    -> Result<Box<[T]>,E>
    {
        self.split_whitespace().map(f).collect()
    }
}

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(),Error> {
    let a = input("> ")?.items(|x| x.parse::<i32>())?;
    println!("{:?}",a);
    Ok(())
}
