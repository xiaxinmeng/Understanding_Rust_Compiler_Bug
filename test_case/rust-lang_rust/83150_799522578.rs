rust
> fn main() {
>     func(0..1)
> }
> 
> fn func<T: Iterator<Item=u8>>(iter: T) {
>     func(iter.map(|x| x+1))
> }
> 