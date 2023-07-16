rust
[1,2,3].iter()
    .zip(std::iter::successors(Some(1), |x| Some(x+1)))
    .rev() // Where to even begin???
