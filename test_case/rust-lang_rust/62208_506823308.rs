rust
for v in h
    .iter_linear(1_000)
    .skip_while(|v| v.quantile() < 0.01)
    .take_until(|v| v.quantile() >= 0.99)
