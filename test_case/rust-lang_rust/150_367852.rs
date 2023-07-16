
import bar = foo.bar
export *;
fn main() -> () {
    let mutable bar.baz x;
    x = bar.a();
}
mod foo {
    export *;
    mod bar {
        export *;
        type baz = rec(uint tag);
    }
}
