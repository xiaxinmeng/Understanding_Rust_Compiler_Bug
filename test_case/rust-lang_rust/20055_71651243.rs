 rust
// #![crate_type="lib"]

pub fn foo(box_1: fn () -> Box<[i8; 1]>,
           box_2: fn () -> Box<[i8; 2]>,
           box_3: fn () -> Box<[i8; 3]>,
           box_4: fn () -> Box<[i8; 4]>) {
    println!("Hello World 1");
    let _: Box<[i8]> = match 3 {
        1 => box_1(),
        2 => box_2(),
        3 => box_3(),
        _ => box_4(),
    };
    println!("Hello World 2");
}

pub fn main() {

    fn box_1() -> Box<[i8; 1]> { Box::new( [1i8] ) }
    fn box_2() -> Box<[i8; 2]> { Box::new( [1i8, 2] ) }
    fn box_3() -> Box<[i8; 3]> { Box::new( [1i8, 2, 3] ) }
    fn box_4() -> Box<[i8; 4]> { Box::new( [1i8, 2, 3, 4] ) }

    foo(box_1, box_2, box_3, box_4);
}
