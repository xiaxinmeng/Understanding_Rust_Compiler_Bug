rust
async fn start<Fut>(prm: for<'a> fn(&'a Vec<String>) -> Fut) 
