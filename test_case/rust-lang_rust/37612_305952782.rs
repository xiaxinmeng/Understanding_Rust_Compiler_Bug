
{ //start scoping.
    match self.cache.read() { //gain a lock and read.
        Ok(ref data) => Some(data) 
        _ => None,
    } //no semi colon so data leaks out of the next scope e.g. it's not unit()
//end scope.
}.map(|data| { //data is available, but the lock is out of scope.
    // use `data` - the lock better be held, but it's not, because you ended the scope, error?
})
