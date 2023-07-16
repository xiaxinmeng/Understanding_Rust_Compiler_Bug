
fn main() -> (){
    ...
    let _1: &dyn std::fmt::Debug;    // "x" in scope 2 at src/main.rs:2:9: 2:10
    let mut _2: &i32;
    let mut _3: &i32;

    bb0: {                              
        ...
        _3 = &(promoted[0]: i32);
        _2 = _3;
        _1 = move _2 as &dyn std::fmt::Debug (Unsize);
        ...
    }
}
