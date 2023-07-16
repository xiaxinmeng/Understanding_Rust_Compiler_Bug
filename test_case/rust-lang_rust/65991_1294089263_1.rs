rust
> trait Shape { fn area(&self) -> f64; }
> trait Circle where Self: Shape { fn radius(&self) -> f64; }
> 