rust
library/std/src/io/mod.rs
///     let mut f = File::open("foo.txt")?;
--
///     let mut buffer = Vec::new();
///     // read the whole file
///     f.read_to_end(&mut buffer)?;
--
    ///     let mut f = File::open("foo.txt")?;
--
    ///     let mut buffer = Vec::new();
    ///
    ///     // read the whole file
    ///     f.read_to_end(&mut buffer)?;

library/stdarch/examples/hex.rs
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
