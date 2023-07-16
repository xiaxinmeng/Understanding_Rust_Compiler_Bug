 rust
fn main(){
    fn bar<'a, T:'a> (t: T) -> Box<FnOnce<(),T> + 'a> {
        box move |:| t
    }
    bar(());
}
