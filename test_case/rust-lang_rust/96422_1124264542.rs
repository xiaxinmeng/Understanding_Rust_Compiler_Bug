rust
match rwlock.read() {
    Ok(_) => …,
    Err(poison_error) => {
        let guard = poison_error.into_inner();
        /* :( */  //RwLock::clear_poison(&guard);

        // instead we have to do:
        drop(guard);
        let another_guard = rwlock.write().unwrap_err().into_inner(); // probably still poisoned (?)
        RwLock::clear_poison(&another_guard);
        drop(another_guard);
        // back to read, since we didn't actually want write to begin with
        let yet_another = rwlock.read().unwrap(); // probably not already re-poisoned (?)
        // now we can finally read
    }
}
