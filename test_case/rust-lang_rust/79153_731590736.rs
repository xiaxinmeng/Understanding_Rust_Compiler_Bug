console
$ python3 -c 's = "fn main() {"; s += "\n".join("let x = 0;" for _ in range(6000)); s+= "}"; print(s)' > a.rs
$ rustc -Aunused -g a.rs
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow
