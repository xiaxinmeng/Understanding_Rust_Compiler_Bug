 rust
($init: expr, .. $n: expr) => {{
    let n = $n;
    let v = Vec::with_capacity(n);
    for _ in range(0, n) { v.push($init) }
    v
}}
