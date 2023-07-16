
let x = { 
    match self.cache.read() { 
        Ok(ref data) => Some(data) 
        _ => None,
    } 
}
//this deadzone is worrying though.
x.lock().map(|data| {
    // use `data` - the lock better be held
})
