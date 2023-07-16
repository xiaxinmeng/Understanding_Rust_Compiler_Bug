 rustc
fn main() {
    let tup = (true, true);
    println( match tup {
             (false, false) => "foo",
             (false, true) => "bar",
             (true, true) => "baz"
             }
           );
}
