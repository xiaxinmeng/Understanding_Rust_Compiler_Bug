rust
let mut fac = one();

(0..).map(move |n: usize| { 
    if n > 0 { 
        fac *= n;
    }
    fac.clone()
})
