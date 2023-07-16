
fn foo<F>(f: F) where F: for<'a> Fn(&'a str) -> &'a str {}
fn bar<F>(f: F) where F: Fn(&str) -> &str {}

fn main() {
//    foo(|a: &str| a); // Compiler panic
    bar(|a: &str| a); // Works

    let local = |a: &str| a;
    bar(local);  // compile failed
}
