`rust
pub  struct v {
    val:[i32;17]
}


pub fn test(a:v, b:v) -> v {
    let mut res = v { val : [0;17] };

    for i in 0..17 {
        res.val[i] = a.val[i] + b.val[i];
    }
    return res;
}

