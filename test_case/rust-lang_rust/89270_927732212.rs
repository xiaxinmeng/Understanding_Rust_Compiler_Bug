
> use std::fs::File;
> use std::io::Write;
> 
> fn main() {
>     let mut file =
>         File::create(r"\\?\..").expect("create dot file");
>     file.write_all(b"Hello, world!").expect("write dot file");
> }
> 