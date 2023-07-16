rust
pub fn id<'a, T>(x: T) -> T { x }
pub fn id_ref<'a, T: 'a>(x: &'a mut T) -> &'a mut T { x }

fn main() {
    let p = &mut 0i32;
    {
        let q = id_ref(p); // ok
        // let q = id(&mut *p); // ok
        // let q = id(p); // error[E0382]: borrow of moved value: `p`
    } // q dies
    println!("{}", p);
}
