
// Reader: okay, that makes sense. how do I make my resource into a thing that can defer dropping?
// Reader: *looks up definition of DeferredDrop*
// Reader: oh, okay, my resource can already defer dropping, and this is here for technical reasons
fn defer_drop(value: Box<dyn DeferredDrop>) { ... }
