
//Consumes a value if its key exists.
let try_consume = |&: key: &str, consume: &mut FnMut(f64)| {
    match object.get(&key.to_string()) {
        Some(ref json) => consume( json.as_f64().unwrap() ),
        None => ()
    };
};
{
//Same as above but consume creates an Effect that gets pushed onto the item.
//This is in a separate scope because the Closure captures item.effects.effects until it goes out of scope.
let mut try_consume_push = |&mut: key: &str, consume: &Fn(f64) -> Effect| {
    try_consume( key, &mut|&mut: value| item.effects.effects.push( consume( value ) ) );
};
