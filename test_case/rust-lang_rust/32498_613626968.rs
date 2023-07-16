rust
struct L<T> {
    n: Option<T>,
}
type L8<T> = L<L<L<L<L<L<L<L<T>>>>>>>>;
type L64<T> = L8<L8<L8<L8<T>>>>;

fn main() {
    use std::mem::size_of;
    // This prints "128: 1":
    println!("128: {}", size_of::<L64<L64<()>>>()); 
    // This line does not compile: error: overflow representing the type std::option::Option<()>
    println!("129: {}", size_of::<L<L64<L64<()>>>>()); 
}
