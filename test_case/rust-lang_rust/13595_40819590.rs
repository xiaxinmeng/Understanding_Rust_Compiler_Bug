 rust
struct StateMachineIter {
    statefn: fn(&mut StateMachineIter) -> Option<&'static str>
}
