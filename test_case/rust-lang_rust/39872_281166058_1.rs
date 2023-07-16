
pub fn f<'a>(v: &'a Vec<i32>) -> impl Iterator<Item=&'a i32> {
    v.iter()
}
