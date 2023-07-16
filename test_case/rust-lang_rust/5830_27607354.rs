 rust
struct Foo(int, int, int, int);
struct Bar{a: int, b: int, c: int, d: int}

fn main() {
    let Foo(*) = Foo(5, 5, 5, 5);
    //let Bar(*) = Bar{a: 5, b: 5, c: 5, d: 5};                                                                                                                                  
    let Bar{_} = Bar{a: 5, b: 5, c: 5, d: 5};
    //let (*) = (5, 5, 5, 5);                                                                                                                                                    
    //let Foo(a, b, *) = Foo(5, 5, 5, 5);                                                                                                                                        
    //let Foo(*, d) = Foo(5, 5, 5, 5);                                                                                                                                           
    //let (a, b, *) = (5, 5, 5, 5);                                                                                                                                              
    //let (*, c, d) = (5, 5, 5, 5);                                                                                                                                              
    //let Bar{b: b, *} = Bar{a: 5, b: 5, c: 5, d: 5};                                                                                                                            
    let Bar{b: b, _} = Bar{a: 5, b: 5, c: 5, d: 5};
    /*match [5, 5, 5, 5] {                                                                                                                                                       
        [a, *] => { }                                                                                                                                                            
    }*/
    /*match [5, 5, 5, 5] {                                                                                                                                                       
        [*, b] => { }                                                                                                                                                            
    }*/
    /*match [5, 5, 5, 5] {                                                                                                                                                       
        [a, *, b] => { }                                                                                                                                                         
    }*/
    match [5, 5, 5] {
        [a, .._] => { }
    }
    match [5, 5, 5] {
        [.._, a] => { }
    }
    match [5, 5, 5] {
        [a, .._, b] => { }
    }
}
