rust
let res: Result<()> = data.iter()
    .try_fold((), |(),x| writeln!(stdout(), "{}", x));
