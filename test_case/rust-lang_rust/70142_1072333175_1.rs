rust
// don't worry too much about these imports, they provide things like concurrency and timeouts
use async_time::prelude::*;
use async_concurrency::prelude::*;
use async_std::fs;

// flatten `io::Result<io::Result<String>>` to `io::Result<String>`
let fut1 = try {
    fs::read_to_string("some-file.txt")
        .timeout(Duration::from_secs(2))??
};

// This returns `io::Result<String>`
let fut2 = fs::read_to_string("other-file.txt");

// Because both futures now have the same signature, we can `race` them.
let s = (fut1, fut2).race().await?;
