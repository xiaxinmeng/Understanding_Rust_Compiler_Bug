
struct Foo { id:int, name:~str }
struct Bar  { foo:Foo, x:int, y:int }

let b=Bar::new();    
println!("id={:?}", b.id) // a.id is a shortcut for  b.foo.id ... since its unambiguous...
