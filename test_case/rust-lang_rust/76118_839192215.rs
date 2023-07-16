rust
// io::Read does something like this
trait Trait {}
impl Trait for &[u8] {}
impl<T: Trait + ?Sized> Trait for &mut T {}

fn takes_trait(t: impl Trait) {}

fn main() {
    let mut arr = [0, 1, 2, 3];
    
    // currently
    takes_trait(&mut &arr[..]);
    
    // with `.as_slice()` its nice and clear
    takes_trait(&mut arr.as_slice());
    
    // can't use deref coercion in this case
    takes_trait(&mut &arr);
    //          ^^^^^^^^^ the trait `Trait` is not implemented for `&[u8; 4]`
}
