 rust
let mut reader = stdin();
let mut line = ~"";
while { line = reader.read_line().unwrap(); line.as_slice() } != "something"
{
    println!("{}", line);
}
