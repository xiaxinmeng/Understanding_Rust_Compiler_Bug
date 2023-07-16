rust
trait A {
    type T: Iterator<Item = i32>;
}

fn foo<X, Y: Send>() 
where
    X: A<T = Y>
{ 
}
