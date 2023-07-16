rust
fn f<'a>(v: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    let r = &v;
    v.iter().map(|e| if r.len() > 10 { e * 2 } else { e * 4 })
}
