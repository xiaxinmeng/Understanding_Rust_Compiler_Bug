rust

#[derive(Debug)]
struct A(i32);

fn main() {
    let mut v = vec![A(1), A(2)];

    //let first = &v[0];

    v.push(A(6));
    //let first = &v[0]; //all errors go away if I uncomment this, even the "multiple rlib" one
    

    println!("The first element is: {:?}", first);
}
