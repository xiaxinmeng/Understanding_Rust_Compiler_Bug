rust
// modulo dealing with offsets
let mut this = self;
for _ in 1..n {
    this = this.extend(self)?;
}
this
