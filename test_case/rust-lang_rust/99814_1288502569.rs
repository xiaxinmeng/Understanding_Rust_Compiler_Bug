rust
fn test() { // U0
    for<'a> |s: &'a str| { //U1
        let _: &'_ str = s; // `'_` lives in a higher universe
    };
}
