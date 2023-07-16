rust
let foo : Option<bool> = Option::None;
println!("size:{} val:{:08b}",
          std::mem::size_of::<Option<bool>>(),
          unsafe {std::mem::transmute::<Option<bool>, u8>(foo)});
// size:1 val:00000010
