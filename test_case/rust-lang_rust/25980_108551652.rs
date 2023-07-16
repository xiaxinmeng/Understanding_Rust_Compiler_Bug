 rust
trait Good {}
impl Good for .. {}

struct Bad;
impl !Good for Bad {}
