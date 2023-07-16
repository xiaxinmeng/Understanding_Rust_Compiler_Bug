
pure fn call_first((x, y): (&fn(), &fn())) { 
    x(); //~ ERROR access to impure function prohibited in pure context
}
fn main() {}
