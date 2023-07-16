
{ 
    let x = match self.cache.read() { 
        Ok(ref data) => Some(data) 
        _ => None,
    } 
   x.maintainLock? x.lend? //how do you punch the lock through this? guessing you don't or can't. so why does it make sense to have brackets in the first place?
}.map(|data| {
    // use `data` - the lock better be held
})
