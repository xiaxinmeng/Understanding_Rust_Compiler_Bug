rust
trait ObjectSafe {
    // some functions
}
trait NotObjectSafe: ObjectSafe + Sized {
    // more functions
}
