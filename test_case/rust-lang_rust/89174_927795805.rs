rust
use std::fs;
use std::env;
use std::io::Result;

fn main() -> Result<()> {
    // Set the current directory to some long path that exists and is just short of the 259 limit.
    let dir = r"Z:\aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\abcdefghi";
    env::set_current_dir(dir)?;

    // This should be successful.
    let name = "a";
    println!("Creating `{}`", name);
    fs::File::create(name)?;

    // This should fail.
    let name = "abc";
    println!("Creating `{}`", name);
    fs::File::create(name)?;

    Ok(())
}

