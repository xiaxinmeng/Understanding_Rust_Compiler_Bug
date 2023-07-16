
   fn load_symbol<'a, T>(&'a self) -> T<crate: 'a>;
   let x = c.load_symbol::<fn() -> fn() -> ()>();
   let f: (fn() -> fn() -> ())<crate: 'a> = x; // OK
   let g: (fn() -> ())<crate: 'a> = x(); // OK
   let h: fn() -> () = x(); // ERROR!
   