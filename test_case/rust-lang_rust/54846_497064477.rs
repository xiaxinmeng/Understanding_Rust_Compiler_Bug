rust
struct Value;

impl Value {
    const fn new() -> Self { Value }
}

struct StaticContainer(&'static [Value]);
