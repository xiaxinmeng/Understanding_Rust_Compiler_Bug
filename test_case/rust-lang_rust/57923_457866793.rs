rust
struct A;
struct B;

trait X { type Y; }

impl X for <A as X>::Y { type Y = B; }
