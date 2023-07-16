 rust
    fn encode(sig: Self, exp: int) -> Self;
    fn decode(&self) -> (Self, int);
    fn significand(&self) -> Self;
    fn exponent(&self) -> int;
