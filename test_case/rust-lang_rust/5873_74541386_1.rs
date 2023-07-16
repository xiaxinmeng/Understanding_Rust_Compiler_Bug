 rust
   enum Foo {
       Bar = 4,
   }
   const I2: usize = Foo::Bar as usize;
   
   fn main () {
       let x: [i32; I2] = [1, 2, 3, 4];
       println!("{:?}", x);
   }
   