
mod a {
    struct Input;
    
    fn foo(x: Input) { }
}

mod b {
    struct Input(i32);
    
    fn foo(x: Input) { }
}

mod c {
    struct Input {
        boo: ()
    }
    fn foo(x: Input) { }
}

mod d {
    fn foo(x: Input) {  }
}
