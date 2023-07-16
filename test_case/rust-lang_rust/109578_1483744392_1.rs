rust
#![feature(iterator_try_reduce)]

fn fallible(check: usize) -> Result<usize, &'static str> {
    if check > 10 {
        Err("Too large")
    } else {
        Ok(check)
    }
}

fn repro() -> Option<usize> {
    [1, 2, 3]
        .into_iter()
        .try_reduce(|accum, item| {
            let num = fallible(item)?;
            Ok::<_, MyErr>(accum + num)
        })
        .unwrap()
}

#[derive(Debug)]
struct MyErr;

impl From<&str> for MyErr {
    fn from(_: &str) -> MyErr { MyErr }
}

fn main() {
    let num = repro().unwrap();
    println!("{num:?}");
}
