 rust
//allowed
trait T {
    fn foo(i32) { }
}

//allowed
trait T {
    fn foo(& x: &i32) { }
}

//allowed
trait T {
    fn foo(&& x: &&i32) { }
}

//forbidden
trait T {
    fn foo(&&& x: &&&i32) { }
}

//forbidden
trait Matrix {
    fn at((row, col): (u32, u32)) { }
}

//forbidden
extern {
    fn foo(i32); // this is a declaration, so it should be OK to omit the name
}
