
static uniq: ~[u32] = ~[10, 20, 30];

fn foo(v: ~[u32]) {
}

fn main() {
    println(fmt!("%u", uniq.len()));
    println(fmt!("%d", uniq[0] as int));
    println(fmt!("%d", uniq[1] as int));
    println(fmt!("%d", uniq[2] as int));
    println(fmt!("%?", uniq));
    let view: &[u32] = uniq;
    println(fmt!("%?", view));
    foo(uniq); // <-- crashes here in runtime, should be compile-time error 
}
