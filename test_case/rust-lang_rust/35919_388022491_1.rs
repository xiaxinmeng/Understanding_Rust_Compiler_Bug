rust
#![feature(nll)]  // enable nll feature, and the compile error is gone
fn main()
{
    let mut buf : &mut [u8] = &mut [1,2,3];
    {
        let tmp : &mut [u8] = &mut *buf; // enforce re-borrow occurs
        buf = &mut tmp[1..];
    }
    println!("{:?}", buf);
}
