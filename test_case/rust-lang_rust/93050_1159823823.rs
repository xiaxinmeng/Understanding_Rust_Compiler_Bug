rs
//Earlier, for example:
let last : Option<u64>;
let next : u64;
//In logic
if last.is_some_and(|v| v == next) {
    return Err(());
}
