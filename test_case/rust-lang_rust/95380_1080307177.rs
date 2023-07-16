rust
struct ZstToken;
fn returns_value_and_token() -> (u32, ZstToken) { ... }

let mut x = 0;
// With this PR:
(x, ZstToken) = returns_value_and_token();
// Without it:
x = returns_value_and_token().0;
// ^ won't detect if the return type of that function changes in the future
