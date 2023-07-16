rust,no_run `` then it will be compile-checked but not run so the directory you use doesn't have to exist. `fs::read_dir()` also can take a `&str` so you don't need to convert to `Path` first:

